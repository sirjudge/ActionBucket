1) Convert the various interface plugins to have shared base traits to make implementation smoother
2) Probably a better way to only run the interface spawn/despawn methods when the state is changed rather than on tick update or set it to only change once. if only change once maybe keep a component specific bool flag or perhaps bit mask type flags for keeping track of things.
