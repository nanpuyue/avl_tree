#[test]
fn insert_test() {
    use super::util::validate;
    use super::*;

    let mut avl_tree = None;
    for i in vec![
        349, 48, 235, 156, 350, 279, 53, 494, 179, 81, 224, 402, 1, 444, 79, 395, 129, 474, 254,
        41, 196, 113, 180, 163, 140, 119, 9, 342, 225, 328, 292, 317, 262, 239, 134, 33, 190, 178,
        99, 361, 226, 107, 162, 148, 358, 318, 334, 371, 69, 389, 66, 187, 39, 495, 283, 49, 233,
        211, 469, 42, 477, 497, 12, 220, 63, 383, 37, 420, 315, 75, 372, 160, 452, 353, 184, 116,
        354, 368, 68, 80, 111, 30, 326, 464, 449, 13, 382, 165, 336, 447, 263, 282, 325, 193, 435,
        104, 386, 212, 222, 321, 45, 244, 186, 92, 26, 434, 451, 55, 352, 229, 44, 309, 281, 103,
        296, 223, 426, 307, 131, 411, 14, 378, 161, 299, 158, 159, 106, 425, 214, 219, 83, 405,
        291, 298, 422, 316, 332, 28, 269, 491, 492, 215, 8, 194, 266, 202, 245, 290, 343, 381, 499,
        319, 155, 293, 338, 271, 380, 101, 40, 110, 363, 123, 121, 6, 313, 19, 259, 284, 46, 448,
        441, 478, 54, 482, 90, 97, 52, 450, 43, 57, 392, 278, 150, 182, 306, 496, 130, 467, 486,
        228, 428, 348, 23, 95, 108, 209, 305, 96, 35, 70, 252, 397, 427, 232, 475, 85, 355, 109,
        264, 207, 275, 91, 406, 248, 416, 295, 369, 201, 236, 466, 74, 136, 465, 417, 47, 460, 498,
        210, 216, 11, 429, 65, 246, 310, 410, 445, 221, 88, 31, 320, 260, 200, 333, 247, 401, 337,
        249, 127, 270, 365, 303, 468, 145, 339, 137, 403, 455, 430, 149, 436, 133, 188, 142, 367,
        268, 323, 172, 439, 89, 456, 117, 288, 217, 73, 82, 267, 396, 442, 379, 205, 185, 471, 274,
        243, 340, 204, 166, 359, 195, 424, 126, 376, 227, 490, 141, 124, 457, 144, 308, 479, 480,
        322, 384, 280, 51, 29, 294, 437, 135, 192, 364, 484, 362, 390, 255, 72, 38, 60, 341, 273,
        177, 241, 2, 86, 198, 314, 276, 432, 25, 256, 22, 151, 183, 433, 472, 18, 17, 206, 453,
        327, 238, 487, 115, 414, 3, 16, 431, 301, 400, 360, 347, 286, 285, 203, 377, 470, 32, 344,
        189, 443, 404, 419, 485, 302, 164, 61, 146, 84, 143, 357, 34, 385, 152, 114, 5, 500, 56, 4,
        176, 312, 139, 93, 483, 311, 399, 488, 174, 71, 7, 181, 167, 459, 105, 297, 132, 253, 463,
        237, 128, 370, 375, 87, 388, 421, 21, 412, 366, 102, 458, 476, 76, 230, 394, 122, 77, 59,
        446, 50, 231, 94, 277, 289, 171, 191, 265, 261, 356, 138, 398, 374, 120, 330, 345, 67, 409,
        481, 157, 250, 329, 173, 335, 118, 78, 197, 125, 407, 15, 408, 258, 153, 438, 234, 304, 36,
        454, 169, 62, 218, 175, 10, 287, 112, 100, 415, 423, 20, 257, 98, 373, 213, 440, 251, 27,
        351, 346, 462, 461, 300, 58, 473, 391, 493, 387, 208, 393, 324, 489, 154, 199, 242, 240,
        331, 413, 418, 24, 64, 168, 147, 170, 272,
    ]
    .into_iter()
    {
        avl_tree.insert(i);
    }
    assert!(validate(&avl_tree));
}