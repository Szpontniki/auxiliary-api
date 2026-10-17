{
	description = "Rust Development Environment";

	inputs = {
		nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
	};

	outputs = { self, nixpkgs }:
		let
			system = "x86_64-linux"; # Adjust if using a different architecture.
			pkgs = import nixpkgs { inherit system; };

			# Packages shared between dev shell and build shell.
			commonPackages = with pkgs; [
				pkg-config
				openssl
				rustc
				cargo
				openapi-generator-cli
				diesel-cli
				libpq
			];
		in
		{
			devShells.${system} = {
				default = pkgs.mkShell {
					packages = commonPackages ++ (with pkgs; [
						rust-analyzer
						docker
					]);
				};

				build = pkgs.mkShell {
					packages = commonPackages;
				};
			};
		};
}
