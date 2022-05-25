# Counting Chromoes

You have washed ashore on a deserted island. After a bit of exploration, you find that the only other inhabitants of this island are a strange bunch of creatures you have decided to call "Chromoes". They're not carnivorous, thank goodness, but they're fiercely territorial and protective of their feeding grounds, which also happen to be the only sources of food on the island. While they're not terribly vicious (you're pretty sure you could take one in a one-on-one throwdown), you don't want to waste the little energy you have left trying to fight a group away from their food source only to fail. You don't think you'll get more than one chance. Thankfully, you've noticed a very ...strange... sort of behavior among these Chromoes. They come in one of three colors: red, green, and blue, and any time two Chromoes of a different color come into contact, they merge into a single Chromo of the third color. This means that, as long as you're patient, you can wait for a group to merge down to a single individual and feel confident in battling for dominance. You'll only get once chance at this, though, so you'd better not guess wrong. You need to devise a way to know for sure that a group of Chromoes will, in fact, merge to a single individual and not leave a small, mono-colored group behind.

Given N Chromoes standing in a line, determine the smallest number of them remaining after any possible sequence of such transformations.

## Business Rules/Errata

- Your input will be given as a string consisting only of the characters 'R', 'G', and 'B'. You should return a number.
- Your input will always contain at least one character.
- You can count on Chromoes who are next to each other in the 'line' to possibly bump and merge, so in the line "RGR", the green Chromo in the middle can merge with either of the red Chromoes.
- You cannot count on Chromoes to push past one another, so in the line "BRR", the red Chromo on the right cannot merge with the blue Chromo, but the middle one can.
- You don't need to determine whether a group _will_ merge to a single individual, only what the smallest possible merged group would be. You have enough time to observe more than one group as long as you don't get froggy and decide to jump too early.

## Example

Given the Chromoes indicated below, you can determine that it is possible to merge down to a single red Chromo:

```
|         Arrangement       |   Change    |  
| --------------------------|-------------|
| ['R', 'G', 'B', 'G', 'B'] | (R, G) -> B |
| ['B', 'B', 'G', 'B']      | (B, G) -> R |
| ['B', 'R', 'B']           | (R, B) -> G |
| ['B', 'G']                | (B, G) -> R |
| ['R']                     |             |
```

This list also merges down to a single individual:

```
|         Arrangement       |   Change    |  
| --------------------------|-------------|
| ['R', 'R', 'R', 'R', 'B'] | (R, B) -> G |
| ['R', 'R', 'R', 'G']      | (R, G) -> R |
| ['R', 'R', 'B']           | (R, B) -> G |
| ['R', 'G']                | (R, G) -> R |
| ['R']                     |             |
```

This list leaves two red Chromoes, too many to fight off:

```
|         Arrangement       |   Change    |  
| --------------------------|-------------|
| ['R', 'R', 'B', 'B']      | (R, B) -> G |
| ['R', 'G', 'B']           | (G, B) -> R |
| ['R', 'R']                |             |
```

