update project_types
set name = 'prototype'
where id = 1;

update project_types
set name = 'map'
where id = 2;


insert into game_versions (version, type)
values ('SpaceWizards', 'release'),
       ('Corvax', 'release');

with project_types as (
    insert into project_types (name)
        values ('bundle')
        returning id
), loader as (
    insert into loaders (loader)
        values ('launcher')
        returning id
)

insert into loaders_project_types (joining_loader_id, joining_project_type_id)
values ((select * from loader), 1),
       ((select * from loader), 2),
       ((select * from loader), (select * from project_types));
