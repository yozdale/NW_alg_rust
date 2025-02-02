use ndarray::{ Array2};

fn  nwallign(shortseq: &str, longseq: &str) -> i32 {
    //Initialize an array that can be used for scoring
    let mut bases1: Vec<char> = shortseq.chars().collect();
    bases1.insert(0,' ');
    let mut bases2: Vec<char> = longseq.chars().collect();
    bases2.insert(0,' ');
    let mut base_array = Array2::<i32>::zeros((shortseq.len() + 1,longseq.len() + 1    ));
    for i in 0..shortseq.len() + 1 {
        base_array[[i, 0]] = (i as i32) * -2;
    }
    for i in 0..longseq.len() + 1 {
        base_array[[0, i]] = (i as i32) * -2;
    }
    
    
    //Add in scores  Sequence 1 is vertical and Sequence 2 is horizontal
    for i in 1..shortseq.len() + 1 {
        for j in 1..longseq.len() + 1 {
            let left = base_array[[i,j-1]] - 2;
            let  up = base_array[[i - 1, j]] - 2;
            let diag = if bases1[i - 1] == bases2[j - 1] { base_array[[i - 1, j -1]] +1} else {base_array[[i - 1, j -1]] - 1};
            base_array[[i,j]] = std::cmp::max(left, std::cmp::max(up, diag));
        }
    }

       // println!("{:?}", base_array);

    let similarity_score = base_array[[shortseq.len(), longseq.len()]];

    //Traceback starting from bottom right with that corner being the origin of coords
    let mut trace1:Vec<char> = Vec::new();
    let mut trace2:Vec<char> = Vec::new();
    let mut coords:Vec<usize> = vec![bases1.len() - 1, bases2.len() - 1];
    
    while coords[0] > 0  && coords[1] > 0 {
       
        //If the bases are the same record and move diagonally
        if bases1[coords[0]] == bases2[coords[1]] {
            trace1.insert(0, bases1[coords[0]]);
            trace2.insert(0, bases2[coords[1]]);
            //println!("{:?}", [bases1[coords[0]]]);
            coords[0] -= 1;
            coords[1] -= 1;
        // If not find the highest scoring neighbor
        } else {
            let left = base_array[[coords[0], coords[1] - 1]];
            let up = base_array[[coords[0] - 1, coords[1]]];
            let diag = base_array[[coords[0] - 1, coords[1] - 1]];

            if left > up && left > diag {
                
                trace1.insert(0, '-');
                trace2.insert(0, bases2[coords[1]]);
                //println!(" left to {:?}", [bases2[coords[1]]]);
                coords[1] -= 1;
                
                
            }
            else if up > diag && up > left {
                
                trace1.insert(0, bases1[coords[0]]);
                trace2.insert(0, '-');   
                //println!("up to {:?}", [bases1[coords[1]]]);
                coords[0] -= 1;     
            }
            
            else {
                trace1.insert(0, bases1[coords[0]]);
                trace2.insert(0, bases2[coords[1]]);
                //println!("diag to{:?}, {:?}", [bases1[coords[0]]], bases2[coords[1]]);
                coords[0] -= 1;
                coords[1] -= 1;
            }
            }   
    }
            
    println!("Seq1: {:?}", trace1);
    println!("Seq2: {:?}", trace2);
    println!("Similarity score was: {:?}", similarity_score);
    similarity_score
}
fn main() {
    let read1 = "AGCTG";
    let read2 = "ACTG";

    nwallign(read1, read2);

}
