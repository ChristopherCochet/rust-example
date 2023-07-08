// Topic: Result & the question mark operator
//
// Requirements:
// * Determine if an employee can access a building using a digital keycard
// * Employees that can access the building are:
//   * Maintenance crews
//   * Marketing department employees
//   * Managers
// * Other employees that work at the company are:
//   * Line supervisors
//   * Kitchen staff
//   * Assembly technicians
// * Ensure that terminated employees cannot access the building
//   regardless of their position
//
// Notes:
// * Use an enum to represent all types of employees
// * Use a struct to store the employee type and whether they are
//   still employed
// * Use a function that returns a Result to determine if the employee
//   may enter the building
// * Print whether the employee may access the building
//   * Must use a function that utilizes the question mark operator to do this


enum Employee_Type {
    Maintenance,
    Marketing, 
    Managers,
    Line_Supervisors,
    Kitchen_Staff,
    Assembly_Technicians,
}

enum Employement_Status {
    Active,
    Terminated, 
}

struct Employee {
    emp_type: Employee_Type,
    emp_status: Employement_Status,
}

fn has_building_access(emp: &Employee) -> Result<(), String> {
    match &emp.emp_status {
        Employement_Status::Terminated => return Err("terminated !".to_owned()),
        _ => (),
    };

    match &emp.emp_type {
        Employee_Type::Maintenance => Ok(()),
        Employee_Type::Marketing => Ok(()),
        Employee_Type::Managers => Ok(()),
        Employee_Type::Line_Supervisors => Err("building access restricted".to_owned()),
        Employee_Type::Kitchen_Staff => Err("building access restricted".to_owned()),
        Employee_Type::Assembly_Technicians => Err("building access restricted".to_owned()),
        _ => Err("building access restricted".to_owned()),
    }
}


fn main() {
    let emp1: Employee = Employee { emp_type: Employee_Type::Managers, emp_status: Employement_Status::Terminated };
    let res = has_building_access(&emp1);
    println!("{:?}",res)
}
