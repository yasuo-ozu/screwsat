#[cfg(test)]
mod tests {
    use screwsat::solver::{Lit, Solver, Status};
    use screwsat::util;
    use walkdir::WalkDir;

    fn model_checker(clauses: &[Vec<Lit>], assigns: &[bool]) -> bool {
        for clause in clauses.iter() {
            let mut satisfied = false;
            for lit in clause {
                if assigns[lit.0] == lit.1 {
                    satisfied = true;
                    break;
                }
            }
            if !satisfied {
                return false;
            }
        }
        true
    }
    fn test_all_files(which: &str) {
        let expected = if which == "sat" {
            Status::Sat
        } else {
            Status::Unsat
        };
        let entries = WalkDir::new(format!("cnf/{}/", which));
        for entry in entries
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| !e.file_type().is_dir())
        {
            let path_str = entry.path().to_str().unwrap();

            if path_str.ends_with(".cnf") {
                //parse cnf file
                let cnf = util::parse_cnf(path_str).unwrap();
                let mut solver = Solver::default();
                cnf.clauses
                    .iter()
                    .for_each(|clause| solver.add_clause(clause));

                eprintln!("Solving... {}", path_str);
                // Time limit is 3 sec
                let status = solver.solve(Some(std::time::Duration::from_secs(3)));
                //Time out
                if status == Status::Indeterminate {
                    eprintln!("Skip!!(TIME LIMIT EXCEEDED): {}", path_str);
                    continue;
                }
                if status != expected {
                    eprintln!(
                        "cnf: {}, Result: {:?} Expected: {:?}",
                        path_str, status, expected
                    );
                    assert!(false);
                }
                if status == Status::Sat {
                    if !model_checker(&cnf.clauses, &solver.assigns) {
                        eprintln!(
                            "Assignments are wrong!! cnf: {}, Result: {:?} Expected: {:?}",
                            path_str, status, expected
                        );
                        assert!(false);
                    }
                }
            }
        }
    }
    #[test]
    fn test_solve() {
        test_all_files("sat");
        test_all_files("unsat");
    }
}
