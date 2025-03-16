# Experiment with CSG::text

Experiment with csgrs text rendering API.

I ran into some issues with the text rendering API in csgrs while
testing in [csgrs-text](https://github.com/winksaville/csgrs-text)

1. There needs to be a char_spacing_scale parameter to `CSG::text` so that
   consecutive characters would be rendered "properly". Below is an example
   of "HHI" rendered and there is no space between the characters:

   ![text_HHI](text_HHI_text_scale-8.131_font-NotoSans-VariableFont_wdth_wght.stl.png)

2. I couldn't easily figure out how to scale the text to a specific size.
   I think the scale parameter should instead be the height of the bounding
   box instead of a scale. That way I'll know the maximum height rendered
   characters would be, making it easier to place the text.

   Here is an example, I wanted to render the characters to be 1mm in high.
   With the current API I had to experiment with the scale factor to get the
   text to the size I wanted. And in this case 8.1307502 was the scale factor
   for "HHI" to be about 1mm high. It would be preferrable if rather than a
   scale we could specify the height in coordinate system units.

   See the extents below from csgrs-text:

   ```
   $ cargo run -- HHI 8.1307502 fonts/NotoSans-VariableFont_wdth_wght.ttf
      Compiling text v0.1.0 (/home/wink/data/prgs/3dprinting/text)
       Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.27s
        Running `target/debug/text HHI 8.1307502 fonts/NotoSans-VariableFont_wdth_wght.ttf`
   text_3d_bb.extents()=[[1.8109243797831487, 1.0, 1.0000000055415066]]
   ```

3. Using height instead of scale would also solve another problem.
   Currently the extents of a bounding box is defined by the characters
   rendered. That means when characters have different heights you might
   have difficulty placing the text.

   Example, below we have "HHi" and it's bounding box height is 1.0322128908740762,
   but the above "HHI" example the bounding box was 1.0000000055415066:
   ```sh
   $ cargo run -- HHi 8.1307502 fonts/NotoSans-VariableFont_wdth_wght.ttf
       Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.06s
        Running `target/debug/text HHi 8.1307502 fonts/NotoSans-VariableFont_wdth_wght.ttf`
   text_3d_bb.extents()=[[1.6470588326565991, 1.0, 1.0322128908740762]]
   ```

   Here is "HHi" rendered:

   ![text_HHi](text_HHi_text_scale-8.131_font-NotoSans-VariableFont_wdth_wght.stl.png)

   So this is another advantage of using height instead of scale.


## Usage

```
Usage: text text_scale font_file
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
