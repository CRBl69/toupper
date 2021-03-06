# toupper

toupper is an online drawing application.

## How it works

It is based on the following mechanisms :

- Everything is an instruction
- Image is stored as an ordered list of instructions
- Rendering is donw by processing all the instructions

### Drawbacks

Storing images as instructions can be a good idea, but there are some drawbacks :

- Very long instruction lists can take forever to load/render
- Filesize can be infinite even for a fixed resolution

### Solutions

Here are some thoughts (that I might or might not implement in the future) that
I thought about in order to fix the above-mentionned problems, along with some
other drawbacks generated by the solutions :

- Render "steps", every `x` instructions, so that you never have to process more than `x` instructions in order to render a result
    - Significally increases filesize
    - Removing an instruction at the begening of the list makes all renders useless
- Truncate instructions at `y` (if we have 150 000 instructions, save the image at the 100 000 instructions state, and the last 50 000 instructions)
    - Lose access to infinite history
