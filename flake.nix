{
	description = "Rust Development Environment";

	inputs = {
		nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
	};

	outputs = { self, nixpkgs }:
		let
			system = "x86_64-linux"; # Adjust if using a different architecture.
			pkgs = import nixpkgs { inherit system; };
		in
		{
			devShells.${system}.default = pkgs.mkShell {
				packages = with pkgs; [
					rustc
					cargo
					rust-analyzer
				];
		};
	};
}
