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

SELECT
S0.system_id AS S0_id,
S0.system_name AS S0_name,
S1.system_id AS S1_id,
S1.system_name AS S1_name,
CASE WHEN S1.observatory IS NULL THEN 0 ELSE 1 END AS S1_JO,
S2.system_id AS S2_id,
S2.system_name AS S2_name,
CASE WHEN S2.observatory IS NULL THEN 0 ELSE 1 END AS S2_JO,
S3.system_id AS S3_id,
S3.system_name AS S3_name,
CASE WHEN S3.observatory IS NULL THEN 0 ELSE 1 END AS S3_JO,
S4.system_id AS S4_id,
S4.system_name AS S4_name,
CASE WHEN S4.observatory IS NULL THEN 0 ELSE 1 END AS S4_JO,
S5.system_id AS S5_id,
S5.system_name AS S5_name,
CASE WHEN S5.observatory IS NULL THEN 0 ELSE 1 END AS S5_JO
FROM named_systems S0
JOIN stargates SG0 ON SG0.system_id = S0.system_id
JOIN named_systems S1 ON SG0.dst_system_id = S1.system_id
JOIN stargates SG1 ON SG1.system_id = S1.system_id
JOIN named_systems S2 ON SG1.dst_system_id = S2.system_id AND S2.system_id != S0.system_id
JOIN stargates SG2 ON SG2.system_id = S2.system_id
JOIN named_systems S3 ON SG2.dst_system_id = S3.system_id AND S3.system_id NOT IN (S0.system_id, S1.system_id)
JOIN stargates SG3 ON SG3.system_id = S3.system_id
JOIN named_systems S4 ON SG3.dst_system_id = S4.system_id AND S4.system_id NOT IN (S0.system_id, S1.system_id, S2.system_id)
JOIN stargates SG4 ON SG4.system_id = S4.system_id
JOIN named_systems S5 ON SG4.dst_system_id = S5.system_id AND S5.system_id NOT IN (S0.system_id, S1.system_id, S2.system_id, S3.system_id)
WHERE
(
(S1.observatory IS NOT NULL AND S2.observatory IS NULL AND S3.observatory IS NULL AND S4.observatory IS NULL AND S5.observatory IS NULL) OR
(S1.observatory IS NULL AND S2.observatory IS NOT NULL AND S3.observatory IS NULL AND S4.observatory IS NULL AND S5.observatory IS NULL) OR
(S1.observatory IS NULL AND S2.observatory IS NULL AND S3.observatory IS NOT NULL AND S4.observatory IS NULL AND S5.observatory IS NULL) OR
(S1.observatory IS NULL AND S2.observatory IS NULL AND S3.observatory IS NULL AND S4.observatory IS NOT NULL AND S5.observatory IS NULL) OR
(S1.observatory IS NULL AND S2.observatory IS NULL AND S3.observatory IS NULL AND S4.observatory IS NULL AND S5.observatory IS NOT NULL)
)
ORDER BY S1_JO DESC, S2_JO DESC, S3_JO DESC, S4_JO DESC, S5_JO DESC;