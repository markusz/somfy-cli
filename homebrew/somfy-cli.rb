class SomfyCli < Formula
  desc "Command-line interface for controlling Somfy smart home devices"
  homepage "https://github.com/markusz/somfy-cli"
  version "0.1.0"
  
  on_macos do
    if Hardware::CPU.intel?
      url "https://github.com/markusz/somfy-cli/releases/download/v#{version}/somfy-cli-v#{version}.tar.gz"
      sha256 "UPDATE_ME_AFTER_FIRST_RELEASE" # This will be updated when you create your first release
    elsif Hardware::CPU.arm?
      url "https://github.com/markusz/somfy-cli/releases/download/v#{version}/somfy-cli-macos-aarch64"
      sha256 "UPDATE_ME_AFTER_FIRST_RELEASE" # This will be updated when you create your first release
    end
  end

  on_linux do
    if Hardware::CPU.intel?
      url "https://github.com/markusz/somfy-cli/releases/download/v#{version}/somfy-cli-linux-x86_64"
      sha256 "UPDATE_ME_AFTER_FIRST_RELEASE" # This will be updated when you create your first release
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