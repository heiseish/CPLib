// pub struct Graph {
//     g: Vec<Vec<usize>>,
// }

// impl Graph {
//     fn new(n: usize) -> Self {
//         Self {
//             g: vec![Vec::new(); n],
//         }
//     }

//     fn rev(g: &Graph) -> Self {
//         let mut s = Self {
//             g: vec![Vec::new(); g.len()],
//         };
//         for i in 0..g.len() {
//             for neighbor in g.get_neighbors(i) {
//                 s.add_edge(*neighbor, i);
//             }
//         }
//         s
//     }

//     #[inline(always)]
//     fn add_edge(&mut self, x: usize, y: usize) {
//         self.g[x].push(y);
//     }

//     #[inline(always)]
//     fn get_neighbors(&self, x: usize) -> &Vec<usize> {
//         &self.g[x]
//     }

//     #[inline(always)]
//     fn len(&self) -> usize {
//         self.g.len()
//     }
// }

type Graph = Vec<Vec<usize>>;
