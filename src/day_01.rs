pub fn solve_puzzle_1() -> i32 {
    return solve(get_numbers(), 2, 2020);
}

pub fn solve_puzzle_2() -> i32 {
    return solve(get_numbers(), 3, 2020);
}

fn solve(numbers: Vec<i32>, n: usize, target: i32) -> i32 {
    // Initialize a vector of length n with zeros for the values
    let mut indices = vec![0; n];

    // Loop over all combinations of indices
    while indices[0] < numbers.len() {
        // Get the set of numbers for this combination
        let nums: Vec<i32> = indices.iter().map(|i| numbers[*i]).collect();

        // Return the product of the numbers if the sum of the numbers matches the target
        let sum: i32 = nums.iter().sum();
        if sum == target {
            return nums.iter().product();
        }

        // Increment the indices to produce the next unique set
        for i in (0..n).rev() {
            if indices[i] + 1 == numbers.len() {
                if i == 0 {
                    // There are no more indices to increment
                    break;
                }
                // The index has reached its max value, wrap it back to zero
                indices[i] = 0;
            } else {
                // We have found an index to increment
                indices[i] += 1;
                break;
            }
        }
    }
    return -1;
}

fn get_numbers() -> Vec<i32> {
    let example = vec![1721, 979, 366, 299, 675, 1456];
    let numbers = vec![
        1753, 1976, 1574, 308, 1384, 1191, 1731, 1829, 1658, 1908, 1663, 2001, 1298, 1888, 1134,
        1213, 965, 2009, 1071, 1591, 1402, 1184, 1836, 1536, 1038, 1871, 1354, 1149, 1863, 1728,
        1896, 1599, 1556, 1222, 1909, 1858, 1754, 1947, 1907, 1656, 1135, 1845, 1504, 1473, 1401,
        1700, 1067, 1790, 1783, 1539, 1087, 1614, 1856, 1895, 1564, 1106, 1204, 1492, 1361, 1897,
        1977, 1210, 1867, 1797, 1232, 1148, 1520, 1989, 210, 1259, 570, 1512, 1894, 1309, 1154,
        1327, 1817, 1875, 1702, 1885, 1664, 1220, 1208, 2000, 1178, 1423, 1454, 1780, 1710, 1362,
        1816, 1491, 1363, 1478, 1648, 1163, 1554, 1195, 1500, 1320, 1698, 1636, 1097, 1573, 1846,
        1747, 1138, 1083, 1505, 1387, 1900, 1143, 1905, 1826, 1735, 1496, 1687, 1704, 1916, 1991,
        1750, 1637, 1742, 691, 1967, 1272, 1657, 1140, 1070, 1985, 1405, 1959, 1218, 1878, 1340,
        1722, 2003, 1258, 1726, 1766, 1868, 1714, 1463, 2006, 1537, 1570, 1526, 1578, 1744, 1734,
        1325, 196, 1935, 1849, 1424, 1972, 1602, 1859, 1341, 1177, 1901, 1902, 1247, 2004, 1350,
        1965, 1407, 836, 1899, 1804, 975, 1510, 1898, 1560, 1777, 1523, 1822, 1830, 1855, 1839,
        1482, 1661, 1835, 1343, 1278, 1449, 1136, 1732, 2008, 1686, 1775, 1952, 1444, 1499, 1680,
        1752, 1597, 1963, 1117, 776,
    ];
    return numbers;
}
