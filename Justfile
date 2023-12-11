# Create new prob from template
prob num:
  @if {{ path_exists("prob" + num) }}; then \
    echo "Path exists"; \
  else \
    echo "Creating prob{{num}}"; \
    cp -r .template prob{{num}}; \
    cargo init prob{{num}}; \
    cat prob1/Cargo.toml | tail -1 >> prob{{num}}/Cargo.toml; \
  fi
