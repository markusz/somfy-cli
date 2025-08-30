class SomfyCli < Formula
  desc "Command-line interface for controlling Somfy smart home devices"
  homepage "https://github.com/markusz/somfy-cli"
  version "0.2.0"
  
  on_macos do
    if Hardware::CPU.intel?
      url "https://github.com/markusz/somfy-cli/releases/download/v#{version}/somfy-cli-v#{version}.tar.gz"
      sha256 "fb2d03e5b166eb64eec453bb7f50bba89621eef3ba1f2f9e6bb875477b71b32d"
    elsif Hardware::CPU.arm?
      url "https://github.com/markusz/somfy-cli/releases/download/v#{version}/somfy-cli-macos-aarch64"
      sha256 "7de1bc8600a74e2ac72afe059aff145a2545d53fa869a307f35d73ddc6490a60"
    end
  end

  on_linux do
    if Hardware::CPU.intel?
      url "https://github.com/markusz/somfy-cli/releases/download/v#{version}/somfy-cli-linux-x86_64"
      sha256 "1e984d2f59620af63ba8795b86f62341cebe5bea67960146a44a2f2329379188"
    end
  end

  license "MIT"

  def install
    if OS.mac?
      if Hardware::CPU.intel?
        # Extract from tarball
        system "tar", "-xzf", cached_download
        bin.install "somfy-cli-macos-x86_64" => "somfy"
      elsif Hardware::CPU.arm?
        bin.install cached_download => "somfy"
      end
    elsif OS.linux? && Hardware::CPU.intel?
      bin.install cached_download => "somfy"
    end
    
    chmod "+x", bin/"somfy"
  end

  test do
    assert_match "somfy-cli", shell_output("#{bin}/somfy --version")
    assert_match "Somfy CLI", shell_output("#{bin}/somfy --help")
  end
end
