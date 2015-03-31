     A
      012345
    0 .\.|./
    1 ..\|/.
    2 ---x--
    3 ../|\.
    4 ./.|.\
    5 /..|..

     B
      012345
    0 .|.../
    1 .|../.
    2 .|./..
    3 \|/...
    4 -x----
    5 /|\...

     C
      012345
    0 .\..|.
    1 ..\.|.
    2 ...\|/
    3 ----x-
    4 .../|\
    5 ../.|.

     A:
      pos     : (3,2)
      dia-min : (1,0)
      dia-max : (5,0)
     B:
      pos     : (1,4)
      dia-min : (0,3)
      dia-max : (5,0)
     C:
      pos     : (4,3)
      dia-min : (1,0)
      dia-max : (5,2)


     (x,y) - A + r(1,1) = 0  | max(A) <= s
     (x,y) - A + r(1,-1) = 0 | max(A) <= s

 Position has an diagonal-minimal origin. i.e., the closest point to the actual
 origin which falls along a diagonal that aims toward the origin (the
 'backslash' diagonal), and a diagonal-maximal origin position. i.e., the
 closest point to the origin which falls along a diagonal that aims *away*
 from the origin and *toward* the (0,s) square, where `s` is the size of the
 board.
 The diagonal-minimal origin for a position (x,y) is equal to 
     (x - min(x,y), y - min(x,y))
 The diagonal-maximal origin for a position (x,y)


 The dia-min maintains that, for P = (x,y), dia-min (x',y'), |x' - y'| == |x - y|
 The dia-max maintains that, for P = (x,y), dia-max (x',y'), |x' + y'| == |x + y|


