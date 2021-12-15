let
  sources = import ./nix/sources.nix;
  nixpkgs-mozilla = import sources.nixpkgs-mozilla;
  pkgs = import sources.nixpkgs {
    overlays =
      [
        nixpkgs-mozilla
        (self: super:
            let chan = self.rustChannelOf { date = "2021-03-01"; channel = "nightly"; };
            in {
              rustc = chan.rust;
              cargo = chan.rust;
            }
        )
      ];
  };
  naersk = pkgs.callPackage sources.naersk {};
  merged-openssl = pkgs.symlinkJoin { name = "merged-openssl"; paths = [ pkgs.openssl.out pkgs.openssl.dev ]; };
in
naersk.buildPackage {
  name = "kollider-client";
  root = pkgs.lib.sourceFilesBySuffices ./. [".rs" ".toml" ".lock" ".html" ".css" ".png"];
  buildInputs = with pkgs; [ openssl pkgconfig clang llvm llvmPackages.libclang zlib cacert curl ];
  LIBCLANG_PATH = "${pkgs.llvmPackages.libclang}/lib";
  OPENSSL_DIR = "${merged-openssl}";
}
