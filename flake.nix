{
  description = "ahk — an AutoHotkey interpreter written in Rust, with a desktop backend for driving a Linux machine";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-26.05";

  outputs =
    {
      self,
      nixpkgs,
    }:
    let
      version = "0.3.2";
      base = "https://github.com/danalec/ahk/releases/download/v${version}";

      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
      ];

      # One release asset per platform. The digests are the SHA-256 of the
      # archive itself, in SRI form, from the 0.3.2 release's
      # SHA256SUMS; `nix flake check` verifies them against the real
      # downloads.
      assets = {
        x86_64-linux = {
          file = "ahk-v${version}-x86_64-linux-gnu.tar.gz";
          dir = "ahk-v${version}-x86_64-linux-gnu";
          hash = "sha256-7UD5a+LvEDaX/sP8OpIjWxGrhpgpfQP++fpzSkzWXvo=";
        };
        aarch64-linux = {
          file = "ahk-v${version}-aarch64-linux-gnu.tar.gz";
          dir = "ahk-v${version}-aarch64-linux-gnu";
          hash = "sha256-jteFqVCkALg7LUvSnwzpefv5qUfXcie4kbZQwVxmSCw=";
        };
        x86_64-darwin = {
          file = "ahk-v${version}-x86_64-macos.tar.gz";
          dir = "ahk-v${version}-x86_64-macos";
          hash = "sha256-1Scv0MR2ktbdaAS9oCGt8eF1CFSnYOunVSYhbwNqsBM=";
        };
        aarch64-darwin = {
          file = "ahk-v${version}-aarch64-macos.tar.gz";
          dir = "ahk-v${version}-aarch64-macos";
          hash = "sha256-u8yTyY04a7BRnAKT+bTI3HOdm+cWw8S9x02zUyYTOn8=";
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
            runHook postInstall
          '';

          meta = with pkgs.lib; {
            description = "An AutoHotkey interpreter written in Rust, with a desktop backend for driving a Linux machine";
            longDescription = ''
              ahk is an AutoHotkey interpreter written in Rust: a lexer, an
              AST, a parser, a runtime, the built-in names and the class
              library of the dialect of v2.0.19, plus a keyboard, a pointer,
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
