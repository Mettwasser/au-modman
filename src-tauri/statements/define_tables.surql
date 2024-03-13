DEFINE TABLE @tb_mods SCHEMAFULL;
DEFINE FIELD download_url ON TABLE @tb_mods TYPE string;
DEFINE FIELD name ON TABLE @tb_mods TYPE string;
DEFINE FIELD version ON TABLE @tb_mods TYPE string
    ASSERT string::is::semver($value);
DEFINE FIELD created_at ON @tb_mods TYPE datetime;
DEFINE FIELD install_path ON @tb_mods TYPE string;
DEFINE INDEX uniqueIndex ON @tb_mods FIELDS name, version UNIQUE;

DEFINE TABLE @tb_profiles SCHEMAFULL;
DEFINE FIELD name ON @tb_profiles TYPE string;
DEFINE FIELD folder_location ON @tb_profiles TYPE string;
DEFINE FIELD created_at ON @tb_profiles TYPE datetime;
DEFINE FIELD modifications ON @tb_profiles TYPE array<record<@tb_mods>>;
DEFINE INDEX uniqueIndex ON @tb_profiles FIELDS name UNIQUE;