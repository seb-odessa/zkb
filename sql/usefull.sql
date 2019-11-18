select distinct  system_name, constellation_name
from named_killmails  join named_attackers on named_killmails.killmail_id = named_attackers.killmail_id
where named_attackers.ship_id in (47459,34495)
and named_killmails.region_name = "The Citadel"
order by constellation_name, system_name


/* query neighbors constellations */
SELECT own.constellation_id AS own, neighbors.constellation_id AS neighbor
FROM stargates
JOIN systems own ON own.system_id = stargates.system_id
JOIN systems neighbors ON neighbors.system_id = stargates.dst_system_id
WHERE neighbors.constellation_id != own.constellation_id

--explain query plan
SELECT
	own.constellation_id AS own_id,
	own_object.object_name AS own_name,
	neighbors.constellation_id AS neighbor_id,
	neighbors_object.object_name AS neighbor_name
FROM stargates
JOIN systems own ON own.system_id = stargates.system_id
JOIN objects own_object ON own.constellation_id = own_object.object_id
JOIN systems neighbors ON neighbors.system_id = stargates.dst_system_id
JOIN objects neighbors_object ON neighbors.constellation_id = neighbors_object.object_id
WHERE neighbors.constellation_id != own.constellation_id
--and own.constellation_id = 20000448