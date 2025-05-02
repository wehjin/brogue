hang_y = 25;
hang_x = 7;
leg_left = 11;
leg_right = 25;
top_y = 100;
roof_x = 5;

spire_y = 70;
sit_x = 6;
sit_y = 35;

linear_extrude(height = 20, center = true)
polygon(
points=[
    // edge
    [-hang_x,hang_y],
    [-leg_left,0],
    [-leg_right,0],
    [-roof_x,top_y],
    [roof_x,top_y],
    [leg_right,0],
    [leg_left,0],
    [hang_x,hang_y],
    // center
    [0, spire_y],
    [sit_x,sit_y],
    [-sit_x,sit_y],
],
paths=[
    [0,1,2,3,4,5,6,7],
    [8,9,10]
]
);
