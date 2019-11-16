select distinct  system_name, constellation_name
from named_killmails  join named_attackers on named_killmails.killmail_id = named_attackers.killmail_id
where named_attackers.ship_id in (47459,34495)
and named_killmails.region_name = "The Citadel"
order by constellation_name, system_name