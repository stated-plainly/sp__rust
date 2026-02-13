# Stated Plainly :: Rust
Stated plainly, this is a big ole' dumping ground for everything I write in **Rust**.

Truth be told... I don't even really *like* coding in **Rust** but it's safe and performant, so I've kinda settled on it as the sensible **ground floor** for everything I do coding-wise going forward.

Longer-term, I'd like to have a proper crack at writing a language of my own, called **casper**, but that's a long way off.

I'm still very much **figuring out** the syntax, but I'm basically aiming for the versatility and fun of coding in **JS**, but safer:

*js*
```js
let name = 'James';
name = 10;
```

*casper*
```casper
-- manually annotated
res name : text = (James); -- type annotation around constructor e.g. text() isn't needed when the variable being passed to is annotated, hence (James)
name = int(10); -- compiler error

-- inferred from assignment
res name = text(James); -- like with Rust and Zig, if the type isn't manually annotated, whatever the first value assigned to the variable becomes its "type"
name = int(10); -- compiler error
```
