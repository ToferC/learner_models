use crate::models::{Learner, Registration, Location, 
    PhysicalInfrastructure, test_plot,
    DigitalInfrastructure, Personnel, Group, DeliveryRole,
    LearningProduct, Module, Audience, Role, BusinessLine,
    Status, LearningStyle, LearningObjective, ContentType,
    Statement, Verb};

use crate::models::learning_product::{Issue};

pub fn generate_learning_products(
    phys_inf: &Vec<PhysicalInfrastructure>,
    digi_inf: &Vec<DigitalInfrastructure>,
    personnal: &Vec<Personnel>,
) -> Vec<LearningProduct> {
    // Product 1 - online learning

    let mut lp1 = LearningProduct::new(
        100, 
        String::from("Discover Data"),
        String::from("I501"),
        String::from("Discover data is..."), 
        Audience::Employee, 
        Role::All, 
        String::from("#DiscoverData"), 
        BusinessLine::DigitalAcademy,
        500,
        9,
        Status::Pilot,
        0.25,
        0.25,
    );

    // Add modules

    let mut lp1_m1 = Module::new(
        101, 
        String::from("I501-1"), 
        String::from("Intro to data"), 
        String::from("Data is the lifeblood of any organization..."), 
        vec![
            LearningStyle::Watch,
            ],
        vec![
            Audience::Employee,
        ],
        ContentType::Asyncronous, 
        vec![
            LearningObjective::new(0.5, Statement::new(
                Verb::Understand,
                String::from("what data is and how it works"),
                0.3)
            ),
            LearningObjective::new(0.5, Statement::new(
                Verb::Understand,
                String::from("How data is used in the GC"),
                0.3)
            ),
            ], 
        90, 
        0.7,
        vec![Issue::Clear],
    );

    
    lp1_m1.digital_infrastructure_id = Some(0);
    
    let mut lp1_m2 = Module::new(
        102, 
        String::from("I501-2"), 
        String::from("Data in Your Organization"), 
        String::from("Each department has its own strengths and..."), 
        vec![
            LearningStyle::Discuss,
            ],
        vec![
            Audience::Employee,
        ],
        ContentType::OnlineFacilitated, 
        vec![
        LearningObjective::new(0.5, Statement::new(
            Verb::Describe,
            String::from("Examples of the digital standards in use"),
            0.3)
        ),
        LearningObjective::new(0.5, Statement::new(
            Verb::Explain,
            String::from("Why applications of the standards make things better."),
            0.3)
        ),
        ], 
        45, 
        0.8,
        vec![Issue::Relevant],
    );
        
    // Add digital infrastructure index reference
    lp1_m2.digital_infrastructure_id = Some(1);
    
    // Add personnel index reference
    lp1_m2.personnel_ids = Some(vec![1]);
    
    lp1.modules.push(lp1_m1);
    lp1.modules.push(lp1_m2);
    
    
    // Product 2 - in-person learning
    // In person class

    let mut lp2 = LearningProduct::new(
        200, 
        String::from("Intro to Security"),
        String::from("P901"),
        String::from("Security is everyone's business..."), 
        Audience::Employee, 
        Role::All, 
        String::from("#P901"), 
        BusinessLine::GCSkills,
        24,
        9,
        Status::Production,
        0.65,
        0.10,
    );

    // Add modules

    let mut lp2_m1 = Module::new(
        201, 
        String::from("P901-1"), 
        String::from("Security Policy in the GC"), 
        String::from("A solid understanding of the GC policy frame..."), 
        vec![
            LearningStyle::Classroom,
            ],
        vec![
            Audience::Specialist,
        ],
        ContentType::InPersonFacilitated, 
        vec![
            LearningObjective::new(0.5, Statement::new(
                Verb::Describe,
                String::from("The GC policy framework around security"),
                0.3)
            ),
            LearningObjective::new(0.5, Statement::new(
                Verb::Describe,
                String::from("Roles organizational and responsibilities on security"),
                0.3)
            ),
            ], 
        90, 
        0.55,
        vec![Issue::Entertaining]
    );

    let mut lp2_m2 = Module::new(
        202, 
        String::from("P901-2"), 
        String::from("Physical Security Fundameentals"), 
        String::from("Security is critical in the office env..."), 
        vec![
            LearningStyle::Classroom,
            ],
        vec![
            Audience::Specialist,
        ],
        ContentType::InPersonFacilitated, 
        vec![
            LearningObjective::new(0.5, Statement::new(
                Verb::Describe,
                String::from("The key threats and mitigators for physical security"),
                0.3)
            ),
            LearningObjective::new(0.5, Statement::new(
                Verb::Describe,
                String::from("What to do in the case of an event"),
                0.3)
            ),
            ], 
        150, 
        0.9,
        vec![Issue::Useful]
    );

    // Add physical infrastructure
    lp2_m1.physicial_infrastructure_id = Some(0);
    lp2_m2.physicial_infrastructure_id = Some(1);

    // add personnel
    lp2_m1.personnel_ids = Some(vec![0]);
    lp2_m2.personnel_ids = Some(vec![0]);

    lp2.modules.push(lp2_m1);
    lp2.modules.push(lp2_m2);

    // product 3 - event

    let mut lp3 = LearningProduct::new(
        300, 
        String::from("Returning to Work: What you need to know"),
        String::from("E311"),
        String::from("Many people have questions..."), 
        Audience::Leader, 
        Role::All, 
        String::from("#ReturnToWork"), 
        BusinessLine::RespectfulInclusiveWorkplace, 
        3000,
        2,
        Status::Production,
        0.10,
        0.65,
    );

    // Add module

    let mut lp3_m1 = Module::new(
        301, 
        String::from("E311-1"), 
        String::from("Panel Discussion"), 
        String::from("Returning to work in the age of COVID-19..."), 
        vec![
            LearningStyle::Watch,
        ],
        vec![
            Audience::Leader,
        ], 
        ContentType::Event, 
        vec![
            LearningObjective::new(0.5, Statement::new(
                Verb::Understand,
                String::from("The GC plan for return to work"),
                0.3)
            ),
            LearningObjective::new(0.5, Statement::new(
                Verb::Understand,
                String::from("How to work with colleagues and corporate enablers"),
                0.3)
            ),
            ], 
        60, 
        0.90,
        Vec::new(),
    );

    
    lp3_m1.digital_infrastructure_id = Some(2);
    lp3_m1.physicial_infrastructure_id = Some(2);
    lp3_m1.personnel_ids = Some(vec![2]);
    
    lp3.modules.push(lp3_m1);
    
    // Add learning products to vec
    let mut learning_products: Vec<LearningProduct> = Vec::new();

    learning_products.push(lp1.clone());
    learning_products.push(lp2.clone());
    learning_products.push(lp3.clone());

    for lp in &learning_products {
        println!("{:?}", lp);
    };

    learning_products
}