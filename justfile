dev:
    #!/usr/bin/env zsh
    trunk serve & cargo watch -x run && fg

build:
    #!/usr/bin/env zsh
    trunk build --release
    cargo build --release
    
