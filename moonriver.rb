class Moonriver < Formula
  desc "Terminal-based console for Klipper instances via Moonraker WebSocket API"
  homepage "https://moonriver.rs/"
  url "https://github.com/willpuckett/moonriver/archive/refs/tags/v0.1.0.tar.gz"
  sha256 "PLACEHOLDER_SHA256_HASH"
  license "MIT"
  head "https://github.com/willpuckett/moonriver.git", branch: "main"

  depends_on "rust" => :build

  def install
    system "cargo", "install", *std_cargo_args
  end

  test do
    # Test version output
    assert_match version.to_s, shell_output("#{bin}/moonriver --version")
    
    # Test help output
    output = shell_output("#{bin}/moonriver --help")
    assert_match "terminal-based console", output.downcase
  end
end
