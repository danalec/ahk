Add-Type -AssemblyName System.Drawing
Add-Type -TypeDefinition @'
using System;
using System.Text;
using System.Runtime.InteropServices;
using System.Drawing;
using System.Drawing.Imaging;
using System.Collections.Generic;

public class Win {
  [DllImport("user32.dll")] public static extern bool EnumWindows(EnumWindowsProc cb, IntPtr p);
  [DllImport("user32.dll")] public static extern bool EnumChildWindows(IntPtr h, EnumWindowsProc cb, IntPtr p);
  public delegate bool EnumWindowsProc(IntPtr h, IntPtr p);
  [DllImport("user32.dll", CharSet=CharSet.Unicode)] public static extern int GetWindowTextW(IntPtr h, StringBuilder s, int n);
  [DllImport("user32.dll", CharSet=CharSet.Unicode)] public static extern int GetClassNameW(IntPtr h, StringBuilder s, int n);
  [DllImport("user32.dll")] public static extern bool IsWindowVisible(IntPtr h);
  [DllImport("user32.dll")] public static extern bool IsWindowEnabled(IntPtr h);
  [DllImport("user32.dll")] public static extern bool GetWindowRect(IntPtr h, out RECT r);
  [DllImport("user32.dll")] public static extern uint GetWindowThreadProcessId(IntPtr h, out uint pid);
  [DllImport("user32.dll")] public static extern IntPtr GetParent(IntPtr h);
  [DllImport("user32.dll")] public static extern long GetWindowLongW(IntPtr h, int i);
  [DllImport("user32.dll")] public static extern bool PrintWindow(IntPtr h, IntPtr dc, uint flags);
  public struct RECT { public int Left, Top, Right, Bottom; }

  static List<string> lines = new List<string>();

  static string Text(IntPtr h) {
    StringBuilder s = new StringBuilder(512);
    GetWindowTextW(h, s, 512);
    return s.ToString();
  }
  static string Cls(IntPtr h) {
    StringBuilder s = new StringBuilder(512);
    GetClassNameW(h, s, 512);
    return s.ToString();
  }
  static void Walk(IntPtr h, int depth) {
    RECT r;
    GetWindowRect(h, out r);
    string pad = new string(' ', depth * 2);
    lines.Add(string.Format("{0}0x{1:X} class={2} vis={3} en={4} rect=({5},{6} {7}x{8}) parent=0x{9:X} style=0x{10:X} ex=0x{11:X} title='{12}'",
      pad, h.ToInt64(), Cls(h), IsWindowVisible(h), IsWindowEnabled(h), r.Left, r.Top, r.Right - r.Left, r.Bottom - r.Top,
      GetParent(h).ToInt64(), GetWindowLongW(h, -16), GetWindowLongW(h, -20), Text(h)));
    EnumWindowsProc cb = null;
    cb = delegate(IntPtr c, IntPtr p) { Walk(c, depth + 1); return true; };
    EnumChildWindows(h, cb, IntPtr.Zero);
  }

  public static string Of(uint pid) {
    lines = new List<string>();
    EnumWindowsProc cb = null;
    cb = delegate(IntPtr h, IntPtr p) {
      uint owner;
      GetWindowThreadProcessId(h, out owner);
      if (owner == pid) Walk(h, 0);
      return true;
    };
    EnumWindows(cb, IntPtr.Zero);
    return string.Join(Environment.NewLine, lines.ToArray());
  }

  public static IntPtr Biggest(uint pid) {
    IntPtr found = IntPtr.Zero;
    long area = 0;
    EnumWindowsProc cb = null;
    cb = delegate(IntPtr h, IntPtr p) {
      uint owner; GetWindowThreadProcessId(h, out owner);
      if (owner == pid && Cls(h) == "AutoHotkeyGUI") {
        RECT r; GetWindowRect(h, out r);
        long a = (long)(r.Right - r.Left) * (r.Bottom - r.Top);
        if (a > area) { area = a; found = h; }
      }
      return true;
    };
    EnumWindows(cb, IntPtr.Zero);
    return found;
  }

  public static string Save(IntPtr h, string path) {
    RECT r; GetWindowRect(h, out r);
    int w = r.Right - r.Left, ht = r.Bottom - r.Top;
    if (w <= 0 || ht <= 0) return "the window has no size";
    Bitmap bmp = new Bitmap(w, ht, PixelFormat.Format32bppArgb);
    using (Graphics g = Graphics.FromImage(bmp)) {
      IntPtr dc = g.GetHdc();
      bool ok = PrintWindow(h, dc, 2);
      g.ReleaseHdc(dc);
      if (!ok) return "the window did not print";
    }
    bmp.Save(path, ImageFormat.Png);
    bmp.Dispose();
    return "saved " + path + " (" + w + "x" + ht + ")";
  }
}
'@ -ReferencedAssemblies System.Drawing

$exe = $args[0]
$dir = $args[1]
$script = $args[2]
$wait = [int]$args[3]
$out = $args[4]
$flags = @()
if ($args.Length -gt 5) { $flags = $args[5..($args.Length - 1)] }

Push-Location $dir
$p = Start-Process -FilePath $exe -ArgumentList ($flags + $script) -PassThru -WindowStyle Normal
Start-Sleep -Seconds $wait
"=== process {0} running={1}" -f $p.Id, (-not $p.HasExited)
[Win]::Of([uint32]$p.Id)
$h = [Win]::Biggest([uint32]$p.Id)
if ($h -eq [IntPtr]::Zero) {
  "=== no window of the class AutoHotkeyGUI"
} else {
  "=== the window 0x$(([int64]$h).ToString('X'))"
  [Win]::Save($h, $out)
}
if (-not $p.HasExited) { Stop-Process -Id $p.Id -Force }
Pop-Location
