-- Define tables
DEFINE TABLE project SCHEMAFULL;
DEFINE FIELD name ON TABLE project TYPE string;
DEFINE FIELD mcu_type ON TABLE project TYPE string;
DEFINE FIELD created_at ON TABLE project TYPE datetime DEFAULT time::now();

DEFINE TABLE component SCHEMAFULL;
DEFINE FIELD ref_des ON TABLE component TYPE string UNIQUE;
DEFINE FIELD value ON TABLE component TYPE string;
DEFINE FIELD footprint ON TABLE component TYPE string;

DEFINE TABLE net SCHEMAFULL;
DEFINE FIELD name ON TABLE net TYPE string UNIQUE;
DEFINE FIELD net_class ON TABLE net TYPE string OPTIONAL;

DEFINE TABLE contains SCHEMAFULL;
DEFINE FIELD in ON TABLE contains TYPE record<project>;
DEFINE FIELD out ON TABLE contains TYPE record<component>;

-- Define indexes
DEFINE INDEX project_name_idx ON TABLE project COLUMNS name UNIQUE;
DEFINE INDEX component_ref_idx ON TABLE component COLUMNS ref_des UNIQUE;
