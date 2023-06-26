fn main() {
    
    let schema = "
    define
    person sub entity,
      name string,
      age long,
      nationality string;
  
    city sub entity,
      name string,
      country string,
      population long;
  
    country sub entity,
      name string,
      continent string,
      population long;
  
    visited sub relation,
      relates person,
      relates city,
      date datetime,
      duration long;
  
    friendship sub relation,
      relates person1,
      relates person2,
      since datetime,
      level string;
  
    travelplan sub relation,
      relates person,
      relates city,
      relates country,
      start_date datetime,
      end_date datetime,
      purpose string;
  
    feedback sub relation,
      relates person,
      relates city,
      rating long,
      comment string;
  
    tourist sub person;
  
    local sub person;
  
    start_date sub attribute,
      value datetime,
      constraint exclusive;
  
    end_date sub attribute,
      value datetime,
      constraint exclusive;
  
    rating sub attribute,
      value long,
      constraint exclusive 1..5;
  
    comment sub attribute,
      value string;
  
    rule tourist_or_local {
      (tourist: $x, local: $y) isa person;
    }
  
    rule tourist_has_visited {
      (tourist: $x) isa person; 
      (visited: $x, _: $r) isa visited;
    }
  
    rule travelplan_dates {
      (travelplan: $x) isa travelplan;
      $x has start_date $start;
      $x has end_date $end;
      $start < $end;
    }
  
    ";
    println!("Hello, world! {}", schema);
}

// room sub entity;
// living sub relation, relates renter, relates residence, owns date;
// friendship sub relation, relates firstperson, relates secondperson;