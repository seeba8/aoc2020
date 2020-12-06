
/* #### Day 1 #### */
select expense1.expenses * expense2.expenses
from day1 expense1
join day1 expense2 on expense1.expenses + expense2.expenses = 2020 and expense1.expenses <= expense2.expenses

select e1.expenses * e2.expenses * e3.expenses
from day1 e1
, day1 e2
, day1 e3
where e1.expenses + e2.expenses + e3.expenses = 2020
and e1.expenses <= e2.expenses and e2.expenses <= e3.expenses

/* #### Day 2 #### */
with PasswordRequirements as (
select password
, left(character,1) character
, len(password) - len(replace(password,left(character,1),'')) as ActualAmount
, cast(left(amount, charindex('-', amount) - 1) as int) MinAmount
, cast(substring(amount, charindex('-', amount) + 1, len(amount) - charindex('-', amount)) as int) MaxAmount
from day2
)
select count(*)
from PasswordRequirements 
where ActualAmount between MinAmount and MaxAmount

with PasswordRequirements as (
select password
, left(character,1) character
, cast(left(amount, charindex('-', amount) - 1) as int) Pos1
, cast(substring(amount, charindex('-', amount) + 1, len(amount) - charindex('-', amount)) as int) Pos2
from day2
)
select *
from PasswordRequirements
where substring(password, pos1, 1) = character 
and substring(password, pos2 , 1) != character
union all
select *
from PasswordRequirements
where substring(password, pos1 , 1) != character 
and substring(password, pos2 , 1) = character


/* #### Day 3 #### */
with MapRows as (
select ROW_NUMBER() over(order by (select null)) as rownum, Map
from Day3
), 
MapRowsWithPath as (
select *
, substring(Map, (3 * (rownum - 1) ) % len(Map) + 1, 1) Path
, (3 * (rownum - 1) + 1) % len(Map) as idx
from MapRows
)
select *
from MapRowsWithPath
where Path = '#'

--- Part 2
with MapRows as (
select ROW_NUMBER() over(order by (select null)) as rownum, Map
from Day3
), 
MapRowsWithPath1 as (
select *
, substring(Map, (1 * (rownum - 1) ) % len(Map) + 1, 1) Path
, (1 * (rownum - 1) ) % len(Map) + 1 as idx
from MapRows
),
MapRowsWithPath2 as (
select *
, substring(Map, (3 * (rownum - 1) ) % len(Map) + 1, 1) Path
, (3 * (rownum - 1) ) % len(Map) + 1 as idx
from MapRows
),
MapRowsWithPath3 as (
select *
, substring(Map, (5 * (rownum - 1) ) % len(Map) + 1, 1) Path
, (5 * (rownum - 1) ) % len(Map) + 1 as idx
from MapRows
),
MapRowsWithPath4 as (
select *
, substring(Map, (7 * (rownum - 1) ) % len(Map) + 1, 1) Path
, (7 * (rownum - 1) ) % len(Map) + 1 as idx
from MapRows
),
MapRowsWithPath5 as (
select *
, substring(Map,((rownum + 1) / 2) % len(Map) , 1) Path
, ((rownum + 1) / 2) % len(Map)  as idx
from MapRows
where rownum % 2 = 1
)
select (select cast(count(*) as bigint) as ct
from MapRowsWithPath1
where Path = '#') * (
select cast(count(*) as bigint) as ct
from MapRowsWithPath2
where Path = '#'
) * (
select cast(count(*) as bigint) as ct
from MapRowsWithPath3
where Path = '#'
)*(
select cast(count(*) as bigint) as ct
from MapRowsWithPath4
where Path = '#'
)*(
select cast(count(*) as bigint) as ct
from MapRowsWithPath5
where Path = '#'
)

with Ordered as (
select ROW_NUMBER() over(order by (select null)) as rownum, Passport
from Day4
), Attributes as (
select ordered.rownum, x.value
from Ordered
cross apply string_split(replace(passport, char(10), ' '), ' ') x
)
select distinct rownum from Attributes a
where 7 = (
select count(*)
from attributes byr
where byr.rownum = a.rownum
and (value like 'byr%'
or value like 'iyr%' 
or value like 'eyr%'
or value like 'hgt%'
or value like 'hcl%'
or value like 'ecl%'
or value like 'pid%'
)
)


-- Doesn't work: 
with Ordered as (
select ROW_NUMBER() over(order by (select null)) as rownum, Passport
from Day4
), Attributes as (
select ordered.rownum, x.value
from Ordered
cross apply string_split(trim(replace(passport, char(10), ' ')), ' ') x
),
AttributeKeyValues as ( 
select rownum
, left(value, charindex(':', value) - 1) as k
, SUBSTRING(value, charindex(':', value) + 1, len(value) - charindex(':', value)) as v 
from Attributes
)
select distinct rownum
from AttributeKeyValues a
where exists (
select top 1 1
from AttributeKeyValues b
where b.rownum = a.rownum
and b.k = 'byr' and cast(b.v as int) between 1920 and 2002
)
and exists (
select top 1 1
from AttributeKeyValues b
where b.rownum = a.rownum
and b.k = 'iyr' and cast(b.v as int) between 2010 and 2020
)
and exists (
select top 1 1
from AttributeKeyValues b
where b.rownum = a.rownum
and b.k = 'eyr' and cast(b.v as int) between 2020 and 2030
)
and exists (
select top 1 1
from AttributeKeyValues b
where b.rownum = a.rownum
and b.k = 'hgt' and (
b.v like '%cm' and cast(left(b.v, len(b.v) - 2) as int) between 150 and 193
) or (
b.v like '%in' and cast(left(b.v, len(b.v) - 2) as int) between 59 and 76
)
)
and exists (
select top 1 1
from AttributeKeyValues b
where b.rownum = a.rownum
and b.k = 'hcl' and b.v like '#[0-9a-f][0-9a-f][0-9a-f][0-9a-f][0-9a-f][0-9a-f]'
)
and exists (
select top 1 1
from AttributeKeyValues b
where b.rownum = a.rownum
and b.k = 'ecl' and b.v in ('amb', 'blu', 'brn', 'gry', 'grn', 'hzl', 'oth')
)
and exists (
select top 1 1
from AttributeKeyValues b
where b.rownum = a.rownum
and b.k = 'pid' and b.v like '[0-9][0-9][0-9][0-9][0-9][0-9][0-9][0-9][0-9]'
)