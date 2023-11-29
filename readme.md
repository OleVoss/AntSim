# AntSim

AntSim was made during a school assignment.
It is a simple, terminal based application simulating (very basic) the mechanisms of ants finding food.

The underlying concepts is the observation that ants, having found a suitable food source, leave pheromones
on their path back home for others to follow. Given a high enough concentration of said pheromones other ants
will follow these paths, putting down pheromones as well. 

Within the simulation, there are four types of ants:
1. **Seeker** \
These will randomly roam the map looking for food to bring back to their colony.
Once they find something, they turn into the second type.
2. **Returner** \
With nutritious food on their back, returners take the shortest path home or follow a preexisting path, while leaving pheromones on the tiles they pass.
3. **Follower** \
Whenever an ant starts at the colony the simulation decides on the type of the ant. When a high enough pheromone concentration
is reached, the ant will start as a follower. Follower will select their path based on the surrounding concentration,
thus following the paths laid out by the returner ants.
4.  **Noobs** (in-dev name that stuck) \
Seeker that have not found anything in a defined number of steps will behave like returners, without leaving pheromones.

The avoid for ants being stuck on certain paths, although the food source is long depleted.
The pheromones will evaporate at a given rate.

Most parameters (e.g. evaporation rate, required pheromone concentration, amount of pheromones put down) can be configured 
within the UI.

Clone the repo and start the sim with `cargo run`, ideally outside an IDE since some keybindings can cause errors.

> **_NOTE:_**: There are some unintended behavior regarding the ants, but nothing that causes major complications.

### Keybindings
| Key           | Usage                                  |
|---------------|----------------------------------------|
| `1`, `2`, `3` | Select the respective tabs.            |
| `space`       | Start and stop the simulation.         |
| `s`           | Start and step through the simulation. |
| `a`           | Spawn a single ant.                    |
| `Ctrl+a`      | Spawn a bulk of ants.                  |
| `Ctrl+r`      | Reset the simulation.                  |


