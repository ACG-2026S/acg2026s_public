# Applied Computer Graphics 4860-1084

![teaser](doc/rep_image.png)

Lecture at graduate school of information science and technology in the university of Tokyo, spring semester, 2026

UTOL (UTokyo-LMS) (for Zoom URL, Slack and GitHub Classroom invitations): 

- https://utol.ecc.u-tokyo.ac.jp/lms/course?idnumber=2026_4886_4860-1084_01


## Instructors

Dr. Nobuyuki Umetani 
- email: n.umetani@gmail.com
- url: http://www.nobuyuki-umetani.com/
- lab's web page: https://cgenglab.github.io/labpage/en/


Sei Imai (Lecture on May. 18th about game development)
- CEO of I. Meisters(https://i-meisters.com/)


Dr. Anran Qi (Guest lecturer for Apr.27th, parametric curve)
- Project assistant professor at Takeo Igarashi's lab
- https://anranqi.github.io/



## Time

Monday 2nd period, 10:25am - 12:10pm


## Course Description

Computer graphics is a technology to computationally represent objects' geometry, appearance and movement. This course is an introduction to the techniques generally seen in computer graphics. The aim of the course is to get familiar with applied mathematics such as linear algebra, vector analysis, partial differential equations, numerical analysis and optimization through the topics in computer graphics. There are Rust/Unity programming assignments to acquire research-oriented graphics programming skills such as shader programming and Git. 

Topics:
- Visualization of lines/trianges using rasterization/ray casting
- Parametric curves/surfaces
- Affine/homography transformation
- Volume representation and visualization
- Monte Carlo integration
- Continuous optimization (gradient/hessian based)
- Variational mesh deformation



## Lecture Schedule

| Day | Topic | 
|:----|:---|
|(1)<br>Apr. 20| **Introduction**<br>Digital image, Rasterization in 2D, Brezenham's algorithm |  
|(2)<br>Apr. 27| **Parametric curve/surface** <br/>Bézier curve, Splines, Polynomial root finding |  
|(3)<br>May. 7| **Coordinate transformation**<br>Affine/ Homography transformation |  
|(4)<br>May 11| **Rasterization1**<br>Graphics pipeline, Depth buffer method, Simple shading | 
|(5)<br>May 18| **Guest Lecture by Sei Imai (I.Meisters)**<br> Game development | 
|(6)<br>May 22| **Spatial data structure**<br> Bounding volume hierarchy, Kd-tree, Octree  | 
|(7)<br>May 25| **Volume Representation**<br>Implicit modeling, Ray marching, Volume rendering | 
|(8)<br>Jun. 8| **Rasterization2**<br> Monte-Carlo integration for anti-aliasing, Gaussian splatting | 
|(9)<br>Jun. 15| **Ray Casting1**<br>Material model, Rendering equation | 
|(10)<br>Jun. 22| **Ray Casting2**<br> Monte Carlo integration for importance sampling |
|(11)<br>June 29| **Optimization**<br> Gradient descent, Back propagation, Newton's method | 
|(12)<br>July 6| **Laplacian mesh deformation**<br> Sparse linear system | 
|(13)<br>July 13| **Differentiable Rendering**<br>Reynolds transport theorem, Edge sampling, Nvdiffrast | 


## Grading

- 20% lecture attendance
  - Attendance is counted based on writing a secret keyword on LMS. The keyword is announced for each lecture.  
- 80% small assignments
  - see below

## Assignments

There are many small programming assignments. 
To do the assignments, you need to create your own copy of this repository through **GitHub Classroom**.  
These assignments need to be submitted using **pull request** functionality of the GitHub. 
Look at the following document. 

[How to Submit the Assignments](doc/submit.md)

**Below is a tentative schedule. Each assignment will be open and in a class one-by-one**

| Task ID | Title | Thumbnail | Assigned | Due |
|:---|:---|:---|:---|:---|
| task01 | **Rasterization of lines and polygons**<br>Winding number |  |Apr. 27| |
| task02 | **Rasterization of parametric curves**<br> Parametric curve, Polynomial root finding | |May 7| |
| task03 | **Perspectively-correct texture mapping**<br>rasterization of triangle,Barycentric coordinates |  |May 11| |
| task04 | **Shader practice1**<br>Unity, Rendering pipeline |  |May 22| |
| task05 | **Acceleration of geometry computing**<br> Bounding-volume hierarchy |  |May 25| |
| task06 | **Shader practice2**<br>Unity, Ray marching method, CSG modeling, implicit modeling | |Jun. 8| |
| task07 | **Gaussian splatting**<br>Rasterization of Gaussian primitives, Tile-based acceleration, Alpha-blending | |Jun. 15| |
| task08 | **Monte Carlo integration**<br/>Importance sampling |  |Jun. 29| |
| task09 | **Laplacian Mesh Deformation**<br> Quadratic programming, Sparse linear system | |Jul. 6| |

### Policy

- Do the assignment by yourself. Do not share the answers of the assignments.
- Late submission of an assignment is subject to grade deduction.
- Score each assignment will not be open soon (instructor needs to adjust weight of the score later).
- The assignments might not be graded soon.


## Slides

To be added



## Reading Material

### General Computer Graphics

- 🎥 [Introduction to Computer Graphics by Cem Yuksel](https://www.youtube.com/watch?v=vLSphLtKQ0o&list=PLplnkTzzqsZTfYh4UbhLGpI5kGd5oW_Hh)
- 🌐 [Awesome Computer Graphics (GitHub)](https://github.com/luisnts/awesome-computer-graphics)
- 📖 [Fundamentals of Computer Graphics – Shirley & Marschner](https://www.amazon.co.jp/dp/1482229390)
- 📖 [Real-Time Rendering Resources](https://www.realtimerendering.com/index.html)
- 📖 [Physically Based Rendering](https://pbr-book.org/4ed/contents)
- 🌐 [Scratchpixel 2.0](https://www.scratchapixel.com/)

### Parametric Representation
- 🌐 [Bézier Curve](https://pomax.github.io/bezierinfo/)

### Rendering / Ray Tracing
- 🌐 [Ray Tracing in One Weekend series](https://raytracing.github.io/)

### Spatial Data Structures
- 📖 [PBR Book, Ch. 7](https://pbr-book.org/4ed/Primitives_and_Intersection_Acceleration)

### Implicit Modeling
- 🌐 [2D SDF by Inigo Quilez](https://iquilezles.org/articles/distfunctions2d/)

### Geometry / Mesh Processing
- 📖 [Polygon Mesh Processing – Botsch et al.](https://www.amazon.co.jp/dp/1568814267)
- 🌐 [Geometry Processing course – Alec Jacobson](https://github.com/alecjacobson/geometry-processing-csc2520)

### Gaussian Splatting
- 📄 [3D Gaussian Splatting (original paper & resources)](https://repo-sam.inria.fr/fungraph/3d-gaussian-splatting/)

### Differentiable Rendering
- 🌐 [Nvdiffrast](https://nvlabs.github.io/nvdiffrast/)

### Related Courses by the Instructor
- 🌐 [Physics-based Animation 2023S](https://github.com/nobuyuki83/Physics-based_Animation_2023S)
- 🌐 [Physics-based Animation 2021S](https://github.com/nobuyuki83/Physics-based_Animation_2021S)

