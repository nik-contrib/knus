Currently `DecodeScalar` derive is only implemented for enums

# Enums

Only enums that contain no data are supported:
```rust
#[derive(ferrishot_knus::DecodeScalar)]
enum Color {
    Red,
    Blue,
    Green,
    InfraRed,
}
```

This will match scalar values in `kebab-case`. For example, this node decoder:
```
# #[derive(ferrishot_knus::DecodeScalar)]
# enum Color { Red, Blue, Green, InfraRed }
#[derive(ferrishot_knus::Decode)]
struct Document {
    #[ferrishot_knus(child, unwrap(arguments))]
    all_colors: Vec<Color>,
}
```

Can be populated from the following text:
```kdl
all-colors "red" "blue" "green" "infra-red"
```
