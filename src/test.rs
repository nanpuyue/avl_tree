#[test]
fn test() {
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
        assert!(validate(&avl_tree));
    }

    for i in vec![
        429, 65, 369, 480, 116, 52, 462, 349, 421, 6, 189, 194, 348, 145, 359, 166, 342, 228, 250,
        355, 398, 296, 205, 483, 54, 306, 402, 22, 235, 142, 217, 461, 152, 497, 149, 172, 412, 99,
        335, 247, 117, 36, 118, 437, 487, 442, 69, 132, 61, 225, 33, 394, 431, 67, 121, 411, 299,
        151, 58, 238, 208, 254, 4, 234, 328, 327, 484, 291, 459, 244, 255, 128, 435, 325, 264, 276,
        375, 197, 53, 377, 308, 187, 248, 266, 27, 222, 134, 282, 472, 252, 436, 275, 179, 495,
        153, 450, 154, 184, 2, 366, 426, 457, 115, 297, 422, 330, 444, 100, 445, 350, 102, 63, 312,
        200, 176, 279, 318, 76, 376, 146, 14, 246, 131, 126, 141, 336, 258, 178, 464, 476, 346,
        272, 340, 68, 122, 465, 316, 338, 363, 439, 329, 447, 380, 451, 396, 45, 59, 466, 181, 409,
        367, 40, 13, 344, 399, 150, 91, 165, 174, 453, 180, 127, 81, 286, 448, 112, 51, 236, 203,
        31, 202, 368, 98, 469, 493, 169, 302, 46, 319, 60, 278, 293, 274, 89, 50, 211, 251, 408,
        39, 83, 317, 331, 90, 383, 26, 101, 96, 323, 474, 443, 391, 125, 231, 160, 500, 167, 171,
        17, 455, 103, 175, 5, 356, 489, 454, 209, 10, 423, 159, 430, 288, 220, 463, 420, 77, 324,
        364, 415, 241, 452, 80, 475, 204, 111, 245, 182, 339, 28, 280, 16, 354, 224, 300, 491, 129,
        314, 386, 418, 494, 144, 106, 72, 173, 320, 192, 256, 414, 262, 478, 97, 35, 294, 424, 75,
        78, 467, 185, 481, 94, 309, 135, 140, 190, 333, 24, 298, 263, 240, 360, 265, 158, 214, 130,
        29, 371, 416, 413, 136, 456, 170, 427, 120, 147, 393, 433, 471, 434, 207, 156, 261, 401,
        341, 86, 239, 307, 353, 301, 385, 162, 460, 470, 25, 468, 243, 212, 311, 199, 498, 477,
        345, 19, 163, 326, 21, 499, 496, 84, 48, 215, 343, 488, 382, 361, 260, 157, 352, 485, 64,
        1, 57, 221, 404, 82, 378, 18, 223, 268, 438, 193, 313, 37, 55, 270, 87, 425, 432, 143, 74,
        405, 230, 218, 388, 379, 23, 374, 41, 15, 73, 479, 267, 42, 397, 119, 138, 47, 12, 191,
        458, 473, 66, 406, 168, 287, 389, 417, 44, 213, 372, 269, 227, 71, 201, 395, 337, 232, 387,
        196, 188, 229, 123, 219, 283, 490, 109, 20, 139, 9, 88, 216, 351, 7, 79, 38, 110, 428, 114,
        370, 3, 259, 237, 289, 281, 384, 62, 334, 133, 315, 108, 304, 249, 107, 492, 85, 11, 164,
        113, 390, 49, 43, 322, 257, 310, 93, 226, 295, 210, 30, 441, 362, 8, 34, 186, 365, 198,
        321, 403, 392, 56, 161, 195, 292, 290, 410, 373, 70, 285, 407, 347, 95, 358, 381, 32, 332,
        357, 284, 486, 446, 137, 206, 183, 303, 242, 440, 155, 253, 400, 419, 92, 177, 482, 148,
        273, 105, 233, 449, 305, 124, 271, 104, 277,
    ]
    .into_iter()
    {
        assert_eq!(avl_tree.delete(i).unwrap().val, i);
        assert!(validate(&avl_tree));
    }
    assert!(&avl_tree.is_none());
}
