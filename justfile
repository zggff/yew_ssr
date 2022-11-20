run:
    #!/usr/bin/env zsh
    trunk serve & cargo watch -x run && fg

build:
    #!/usr/bin/env zsh
    trunk build --release --public-url ./dist
    cargo build --release
    
