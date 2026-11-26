{
  description = "ahk — an AutoHotkey interpreter written in Rust, with a desktop backend for driving a Linux machine";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-26.05";

  outputs =
    {
      self,
      nixpkgs,
    }:
    let
      version = "0.6.7";
      base = "https://github.com/danalec/ahk/releases/download/v${version}";

      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
      ];

      # One release asset per platform. The digests are the SHA-256 of the
      # archive itself, in SRI form, from the release's
      # SHA256SUMS; `nix flake check` verifies them against the real
      # downloads.
      assets = {
        x86_64-linux = {
          file = "ahk-v${version}-x86_64-linux-gnu.tar.gz";
          dir = "ahk-v${version}-x86_64-linux-gnu";
          hash = "sha256-/Y1c+gcbZJ23mlFq4yYyXk8Ycfp5e6L2/DhyA4T6NrQ=";
        };
        aarch64-linux = {
          file = "ahk-v${version}-aarch64-linux-gnu.tar.gz";
          dir = "ahk-v${version}-aarch64-linux-gnu";
          hash = "sha256-GA3DTwtP+qkU35Snkg1RQd1inASyAVGgOj+IDjlbiqQ=";
        };
        x86_64-darwin = {
          file = "ahk-v${version}-x86_64-macos.tar.gz";
          dir = "ahk-v${version}-x86_64-macos";
          hash = "sha256-WJA4DE7VnAj88ilHX3XuhgJipzsfa/5qhDfHl+w+lVs=";
        };
        aarch64-darwin = {
          file = "ahk-v${version}-aarch64-macos.tar.gz";
          dir = "ahk-v${version}-aarch64-macos";
          hash = "sha256-yNvMisrcc4ujxPIGbSOmW0CbJDqZIAhAOlC/Iok9IvQ=";
        };
      };

      # The archive holds the runner and the licence under a directory named
      # after the release, so the source root is that directory.
      mkAhk =
        pkgs: file: dir: hash:
        pkgs.stdenv.mkDerivation {
          pname = "ahk";
          inherit version;

          src = pkgs.fetchurl {
            url = "${base}/${file}";
            inherit hash;
          };

          # The archive holds a prebuilt ELF/Mach-O binary, so on Linux the
          # interpreter and shared-library paths are rewritten for the store.
          nativeBuildInputs = pkgs.lib.optional pkgs.stdenv.hostPlatform.isLinux pkgs.autoPatchelfHook;
          buildInputs = pkgs.lib.optional pkgs.stdenv.cc.cc.lib;

          sourceRoot = dir;
          dontBuild = true;
          dontFixup = pkgs.stdenv.hostPlatform.isDarwin;

          installPhase = ''
            runHook preInstall
            mkdir -p "$out/bin" "$out/share/doc/ahk" "$out/share/licenses/ahk"
            install -m755 ahk "$out/bin/ahk"
            for f in README.md; do
              if [ -f "$f" ]; then
                install -m644 "$f" "$out/share/doc/ahk/$f"
              fi
            done
            install -m644 LICENSE "$out/share/licenses/ahk/LICENSE"
            # The icons of the theme: the archive of 0.3.2 has none, and
            # the ones after it carry the set of PNGs that the theme of the
            # desktop takes (`share/icons/hicolor/<size>x<size>/apps/ahk.png`).
            if [ -d share/icons ]; then
              mkdir -p "$out/share/icons"
              cp -r share/icons/hicolor "$out/share/icons/hicolor"
            fi
            # The entry of the menu: it names the icon of the theme.
            if [ -f share/applications/ahk.desktop ]; then
              mkdir -p "$out/share/applications"
              install -m644 share/applications/ahk.desktop "$out/share/applications/ahk.desktop"
            fi
            runHook postInstall
          '';

          meta = with pkgs.lib; {
            description = "An AutoHotkey interpreter written in Rust, with a desktop backend for driving a Linux machine";
            longDescription = ''
              ahk is an AutoHotkey interpreter written in Rust: a lexer, an
              AST, a parser, a runtime, the built-in names and the class
              library of the dialect of v2.0.31, plus a keyboard, a pointer,
              screen captures, files, windows and hotkeys.

              The dialect is v1 or v2. The v2 is the default and the v1 lives
              behind a flag, and the v2 has two modes behind flags of their
              own: the mechanisms of the v2.1, and the extensions of
              KeySharp.

                echo 'Msg "hello"' | ahk

              This package installs the binary published on the release; the
              source lives at the homepage.
            '';
            homepage = "https://github.com/danalec/ahk";
            changelog = "https://github.com/danalec/ahk/releases/tag/v${version}";
            license = licenses.asl20;
            mainProgram = "ahk";
            sourceProvenance = [ sourceTypes.binaryNativeCode ];
            platforms = platforms.unix;
          };
        };

      forAllSystems = f: nixpkgs.lib.genAttrs systems (system: f nixpkgs.legacyPackages.${system});

      # SIMD: a build from source with the vector extensions of the machine
      # turned on, beside the portable prebuilt binary. The level is a
      # portable baseline per architecture (AVX2 on x86_64, NEON on ARM),
      # not `native`, so the derivation is reproducible. The default package
      # stays the portable binary; this one is opt-in (`nix build .#ahk-simd`).
      simdFlags = {
        x86_64-linux = "-C target-cpu=x86-64-v3";
        aarch64-linux = "-C target-cpu=neoverse-n1";
        x86_64-darwin = "-C target-cpu=x86-64-v3";
        aarch64-darwin = "-C target-cpu=apple-m1";
      };

      mkAhkSimd =
        pkgs:
        pkgs.rustPlatform.buildRustPackage {
          pname = "ahk-simd";
          inherit version;

          src = self;
          cargoLock.lockFile = ./Cargo.lock;

          # Only the runner is wanted; the flags go to the build, and the
          # build is native (system == target), so the build-scripts run on
          # the same baseline.
          cargoBuildFlags = [ "-p" "ahk-cli" ];
          RUSTFLAGS = simdFlags.${pkgs.stdenv.hostPlatform.system};

          doCheck = false;

          meta = with pkgs.lib; {
            description = "An AutoHotkey interpreter written in Rust (SIMD build from source)";
            homepage = "https://github.com/danalec/ahk";
            license = licenses.asl20;
            mainProgram = "ahk";
            sourceProvenance = [ sourceTypes.fromSource ];
            platforms = platforms.unix;
          };
        };
    in
    {
      packages = forAllSystems (
        pkgs:
        let
          asset = assets.${pkgs.stdenv.hostPlatform.system};
          portable = mkAhk pkgs asset.file asset.dir asset.hash;
        in
        {
          ahk = portable;
          default = portable;
          ahk-simd = mkAhkSimd pkgs;
        }
      );

      apps = forAllSystems (pkgs: {
        default = {
          type = "app";
          program = "${self.packages.${pkgs.stdenv.hostPlatform.system}.default}/bin/ahk";
          meta.description = "An AutoHotkey interpreter written in Rust, with a desktop backend for driving a Linux machine";
        };
      });

      devShells = forAllSystems (pkgs: {
        default = pkgs.mkShell {
          packages = with pkgs; [
            cargo
            rustc
            rustfmt
            rust-analyzer
            clippy
            cargo-nextest
            mold
            pkg-config
          ];
        };
      });

      checks = forAllSystems (
        pkgs:
        let
          ahk = self.packages.${pkgs.stdenv.hostPlatform.system}.default;
        in
        {
          smoke = pkgs.runCommand "ahk-smoke" { nativeBuildInputs = [ ahk ]; } ''
            export HOME=$TMPDIR
            echo 'Msg "hello from nix"' | ahk | grep -q 'hello from nix'
            touch "$out"
          '';
        }
      );
    };
}
