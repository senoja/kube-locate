class KubeLocate < Formula
  desc "CLI tool for navigating different contexts and namespaces"
  homepage "https://github.com/senoja/kube-locate"
  url "https://github.com/senoja/kube-locate/releases/download/v0.1.0/kube-locate-v0.1.0-x86_64-apple-darwin.tar.gz"
  sha256 "7937e8014af0b09982b90f8a25d22a51042ec9ea3022f307d9be12b1d940a54e"
  head "https://github.com/senoja/kube-locate.git"

  bottle :unneeded

  depends_on "kubernetes-cli"

  def install
    bin.install "klo"
  end

  test do
    assert_match "usage:", shell_output("#{bin}/klo -h 2>&1")
  end
end
