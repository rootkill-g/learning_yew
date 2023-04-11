# Learning Yew

Rust has been amazing experience for me, so moving along with Backend to Yew.rs to make full stack applications, I am learning yew and putting all the code here.

# How to run it

To run the Yew application, you will need to add `wasm32-unknown-unknown` target to your environment.


```
rustup target add wasm32-unknown-unknown
```

Then to fire up the server you'll need to install trunk.


```
cargo install --locked trunk
```

then serve the application with:


```
trunk serve
```


which is a really cool way to run this server while developing as trunk will rebuild the application if you make any modifications to it's source files.
