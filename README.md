Day 4 of my Rust journey.

While building the nth Fiboncci program, I came accross the same weirdness around calculating with floats. I went with using `phi` to compute the value, instead of doing the O(n) approach, and had to make everything an `f64`. It seems you can't call `.sqrt()` on an `int`? Why is that? And so after I added `.0` to all my numbers, I had to add `_f64` to `5.0`, to be able to call `.sqrt()` on it. But, the compiler knows that `1.0` is a float?

Seems really odd to have to do all of this. Would appreciate any feedback on this, and help understanding a better way.
