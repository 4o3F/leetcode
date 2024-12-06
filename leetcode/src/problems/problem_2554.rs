use std::collections::HashSet;

impl Solution {
    pub fn max_count(banned: Vec<i32>, n: i32, max_sum: i32) -> i32 {
        let banned = banned
            .into_iter()
            .filter(|&num| num <= n)
            .collect::<HashSet<i32>>();
        // tracing::debug!("banned: {}", banned.len());
        let mut current_sum = 0;
        let mut count = 0;
        for num in 1..=n {
            if banned.contains(&num) {
                continue;
            }
            current_sum += num;
            if current_sum > max_sum {
                return count;
            }
            count += 1;
            // tracing::debug!(
            //     "current_num: {} current_sum: {}, count: {}",
            //     num,
            //     current_sum,
            //     count
            // );
        }
        count
    }
}

struct Solution {}
pub fn run() {
    let v = vec![
        505, 1537, 1346, 1913, 181, 151, 170, 1177, 1257, 113, 1986, 488, 2105, 1313, 1636, 878,
        557, 1305, 1756, 1849, 779, 665, 1731, 159, 905, 2101, 1470, 5, 836, 262, 1482, 1486, 753,
        239, 1189, 105, 546, 2049, 1327, 1595, 161, 712, 623, 1206, 125, 178, 2015, 1732, 471,
        1060, 1087, 1801, 916, 1279, 452, 991, 161, 916, 801, 1637, 1897, 1185, 1774, 228, 783,
        745, 103, 813, 16, 78, 495, 1477, 762, 1121, 2092, 1432, 807, 497, 1933, 77, 1836, 1429,
        711, 1994, 1978, 1764, 1159, 1293, 1140, 1523, 1373, 802, 691, 847, 1774, 1198, 466, 1807,
        1336, 1528, 993, 2080, 890, 94, 342, 1346, 1415, 1906, 968, 1133, 1742, 1106, 1914, 876,
        914, 1226, 40, 1610, 1963, 398, 76, 1294, 766, 52, 785, 1378, 1187, 1207, 1929, 1557, 269,
        62, 1033, 198, 541, 1064, 1186, 660, 1105, 1440, 1418, 406, 1705, 1045, 96, 1062, 1120,
        789, 1277, 1569, 1389, 1686, 1624, 1641, 256, 1295, 570, 1399, 1651, 1318, 710, 1809, 2012,
        730, 779, 365, 1241, 2038, 592, 892, 475, 125, 1621, 754, 1343, 1063, 1709, 1625, 87, 1959,
        1218, 1438, 1758, 244, 296, 1303, 752, 1460, 807, 852, 2098, 1954, 238, 573, 316, 1682,
        1456, 1855, 1130, 1923, 1625, 813, 1592, 23, 2035, 745, 856, 875, 378, 957, 1691, 653, 467,
        1483, 368, 413, 48, 1632, 529, 914, 322, 2092, 827, 1978, 110, 1041, 147, 1781, 1693, 1874,
        1000, 1907, 497, 1097, 618, 1454, 828, 515, 1790, 297, 1538, 1273, 444, 1116, 1835, 469,
        942, 1364, 465, 444, 179, 247, 1924, 1909, 1276, 214, 949, 1930, 1262, 1948, 1405, 982,
        1313, 1047, 762, 342, 1595, 85, 1287, 1542, 952, 172, 329, 1801, 76, 199, 537, 2070, 709,
        1128, 1604, 196, 861, 669, 928, 865, 940, 698, 2024, 1884, 342, 2060, 1914, 1919, 1662,
        1992, 2009, 329, 358, 290, 452, 1759, 1392, 995, 538, 500, 381, 1757, 2039, 200, 1316, 30,
        450, 700, 1883, 1239, 887, 456, 1180, 2090, 1050, 925, 1727, 591, 940, 750, 713, 701, 1317,
        1374, 1503, 1047, 1586, 1492, 885, 1577, 702, 424, 743, 954, 796, 854, 1638, 191, 992,
        2121, 703, 1280, 469, 1587, 1400, 1660, 1254, 696, 65, 153, 889, 572, 403, 1125, 528, 1131,
        261, 141, 1393, 89, 1585, 1935, 681, 1544, 1394, 1104, 1332, 1655, 324, 13, 689, 1879, 473,
        1676, 1279, 1748, 1172, 33, 590, 819, 2001, 1874, 1565, 1502, 1090, 608, 1213, 736, 543,
        530, 86, 266, 1769, 1879, 1909, 295, 1040, 1521, 323, 311, 547, 1209, 1385, 1605, 677,
        1758, 493, 1789, 197, 1361, 234, 2047, 946, 2081, 597, 2029, 764, 746, 803, 136, 416, 1793,
        459, 36, 1801, 957, 2045, 1663, 1143, 847, 901, 1563, 388, 360, 725, 775, 2102, 1290, 1239,
        2111, 1036, 477, 1621, 750, 595, 145, 1109, 1514, 1672, 1686, 943, 1567, 1606, 1837, 887,
        452, 1104, 1579, 994, 1614, 168, 167, 762, 1929, 790, 1431, 1897, 1696, 64, 1162, 1482,
        402, 1868, 591, 180, 112, 770, 1254, 1868, 924, 1156, 1246, 1790, 394, 348, 530, 1454,
        1371, 1256, 1325, 897, 238, 952, 903, 217, 1245, 304, 410, 1938, 1309, 1471, 88, 123, 1588,
        80, 1629, 748, 145, 288, 2046, 1381, 1171, 1164, 1689, 774, 179, 1525, 742, 1202, 1407,
        1697, 446, 537, 1233, 863, 1019, 1510, 1144, 1338, 2077, 1413, 1228, 1554, 401, 851, 534,
        1324, 1947, 334, 202, 1219, 2066, 1258, 717, 385, 950, 108, 1707, 349, 1442, 1544, 320,
        1605, 1733, 401, 2052, 1395, 65, 398, 2089, 1822, 1307, 224, 2025, 472, 1518, 1949, 1621,
        1148, 175, 1161, 66, 2110, 1709, 200, 818, 1392, 940, 1531, 885, 1843, 270, 942, 1191,
        1039, 1646, 1565, 624, 1803, 833, 1972, 407, 1580, 1160, 710, 360, 687, 1125, 822, 786,
        1208, 1932, 1512, 1459, 1028, 113, 1101, 2092, 493, 615, 831, 1260, 176, 208, 1522, 1624,
        546, 1710, 419, 1427, 330, 48, 692, 574, 2111, 1762, 1747, 1837, 1013, 1315, 1110, 60,
        1493, 401, 164, 651, 1868, 863, 1568, 738, 1858, 1937, 1833, 1240, 1860, 1129, 1794, 315,
        1269, 1809, 263, 27, 446, 479, 1002, 1612, 1678, 623, 70, 893, 1057, 1648, 2026, 264, 921,
        1005, 994, 1929, 877, 641, 755, 941, 1206, 2038, 720, 1496, 511, 1486, 739, 202, 114, 1874,
        745, 1955, 1062, 1979, 803, 989, 749, 951, 121, 553, 1539, 2029, 1160, 780, 1022, 271,
        1120, 436, 91, 1341, 1756, 1154, 1024, 1403, 1248, 1598, 1929, 129, 122, 1184, 1817, 1341,
        1591, 1698, 1084, 174, 1837, 2113, 531, 1988, 166, 1222, 62, 1151, 1861, 1821, 1403, 82,
        148, 235, 675, 958, 2098, 1981, 1885, 1101, 104, 937, 1220, 393, 949, 1204, 822, 98, 842,
        697, 1984, 1695, 1336, 1579, 843, 1739, 927, 1237, 864, 1486, 651, 2073, 1931, 2081, 247,
        1286, 895, 1946, 1690, 24, 295, 1170, 867, 462, 105, 1796, 365, 65, 1517, 1505, 189, 542,
        208, 1264, 1488, 1022, 1181, 1888, 233, 621, 473, 243, 1697, 1496, 618, 958, 1260, 673,
        531, 587, 1125, 1069, 1460, 1550, 1665, 828, 1847, 1804, 725, 791, 768, 193, 87, 340, 857,
        1981, 841, 1585, 1516, 628, 751, 1614, 469, 980, 630, 987, 1574, 1354, 1856, 358, 640,
        1513, 688, 1267, 1864, 1473, 1136, 2103, 1839, 951, 137, 1704, 347, 562, 816, 1630, 1370,
        177, 713, 1000, 711, 110, 824, 227, 1322, 1361, 1706, 305, 1003, 700, 338, 1869, 879, 985,
        1793, 792, 1399, 1339, 1324, 939, 805, 1288, 232, 892, 595, 1234, 2004, 1498, 1159, 124,
        514, 267, 1718, 422, 920, 759, 1362, 422, 360, 1992, 174, 1999, 1146, 1417, 1156, 201, 340,
        1825, 19, 72, 859, 1067, 925, 1303, 637, 674, 2026, 1738, 1633, 1410, 873, 866, 380, 917,
        1823, 350, 2023, 1418, 448, 1595, 165, 1248, 853, 578, 105, 137, 1693, 1812, 2100, 417,
        1048, 141, 2080, 76, 1147, 1981, 111, 1895, 1821, 1677, 997, 675, 528, 602, 1174, 1031,
        2072, 1001, 462, 2114, 668, 1426, 295, 1997, 931, 332, 678, 1180, 1388, 1380, 14, 1699,
        1497, 357, 329, 1575, 1588, 2117, 36, 1568, 1876, 67, 1251, 2113, 780, 2125, 1606, 1679,
        1197, 1467, 882, 647, 66, 790, 311, 287, 1692, 636, 1935, 548, 1930, 960, 1040, 1098, 857,
        1207, 1034, 1405, 1206, 449, 79, 1573, 1681, 984, 823, 1887, 1376, 933, 2105, 1388, 1323,
        1447, 1334, 367, 1778, 717, 1799, 458, 1687, 1429, 1959, 594, 1439, 199, 773, 331, 417,
        1477, 1471, 1713, 678, 656, 2059, 2122, 175, 647, 1938, 1169, 552, 1848, 766, 446, 1200,
        1930, 1844, 13, 711, 798, 1044, 1742, 1057, 737, 986, 129, 337, 1618, 985, 630, 1875, 317,
        1761,
    ];
    tracing::info!("{}", Solution::max_count(v, 416, 445719215));
    tracing::info!("{}", Solution::max_count(vec![1, 6, 5], 5, 6));
}
