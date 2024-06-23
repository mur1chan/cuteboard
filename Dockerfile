FROM nixos/nix:latest

COPY . /app
WORKDIR /app

RUN nix-shell -p 'nixpkgs#nixUnstable' --run "nix build .#nixpacks -o result"

CMD ["./result/bin/cuteboard"]
