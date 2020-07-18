pub fn run () -> Vec<i32> {
    let n = 4;
    let p = vec![
    vec![1,0],
    vec![2,0],
    vec![3,1],
    vec![3,2],
    vec![0,3]
    ];

    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        let num_courses = num_courses as usize;
        let mut adjacency_list: Vec<Vec<usize>> = vec![Vec::new(); num_courses];

        for prereq in prerequisites {
            adjacency_list[prereq[0] as usize].push(prereq[1] as usize);
        }

        let mut no_prereqs: Vec<usize> = Vec::new();
        for i in 0..num_courses {
            if adjacency_list[i].len() == 0 {
                no_prereqs.push(i);
            }
        }

        let mut output = Vec::new();

        while no_prereqs.len() > 0 {
            let course = no_prereqs.remove(0);
            output.push(course as i32);

            for i in 0..num_courses {
                if i == course { continue; }
                for j in 0..adjacency_list[i].len() {
                    if adjacency_list[i][j] == course {
                        adjacency_list[i].remove(j);

                        if adjacency_list[i].len() == 0 {
                            no_prereqs.push(i);
                        }
                        else { break; }
                    }
                }
            }
        }



        for course in adjacency_list {
            if course.len() != 0 {
                return Vec::new();
            }
        }
        output
    }

    find_order(n, p)
}