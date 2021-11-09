import React, { useState } from 'react';
import './Taskview.css'

function Taskview  (){
    const [input, setInput] = useState(''); // '' is the initial state value
// https://stackoverflow.com/questions/36683770/how-to-get-the-value-of-an-input-field-using-reactjs
        return (
            <div className="Taskview">
                <div className="Header">
                    <div>"Header"</div>
                        <div>
                        <label>Please specify:</label>
                        <input value={input} onInput={e => setInput(e.target.value)}/>
                        </div>
                </div>
                <div className="Content">
                    <p>"Content"</p>
                    <p>Applied mechanics is the branch of science concerned with the motion of any substance that can be experienced or perceived by humans without the help of instruments[1]. In short, when mechanics concepts surpass being theoretical and are applied and executed, general mechanics becomes applied mechanics. It is this stark difference that makes applied mechanics an essential understanding for practical everyday life[2]. It has numerous applications in a wide variety of fields and disciplines, including but not limited to structural engineering, astronomy, oceanography, meteorology, hydraulics, mechanical engineering, aerospace engineering, nanotechnology, structural design, earthquake engineering, fluid dynamics, planetary sciences, and other life sciences[3][4]. Connecting research between numerous disciplines, applied mechanics plays an important role in both science and engineering[1].

Pure mechanics describes the response of bodies (solids and fluids) or systems of bodies to external behavior of a body, in either a beginning state of rest or of motion, subjected to the action of forces. Applied mechanics bridges the gap between physical theory and its application to technology.

Composed of two main categories, Applied Mechanics can be split into classical mechanics; the study of the mechanics of macroscopic solids, and fluid mechanics; the study of the mechanics of macroscopic fluids[4]. Each branch of applied mechanics contains subcategories formed through their own subsections as well[4]. Classical mechanics, divided into statics and dynamics, are even further subdivided, with statics' studies split into rigid bodies and rigid structures, and dynamics' studies split into kinematics and kinetics[4]. Like classical mechanics, fluid mechanics is also divided into two sections: statics and dynamics[4].

Within the practical sciences, applied mechanics is useful in formulating new ideas and theories, discovering and interpreting phenomena, and developing experimental and computational tools[5]. In the application of the natural sciences, mechanics was said to be complemented by thermodynamics, the study of heat and more generally energy, and electromechanics, the study of electricity and magnetism.


Contents
1	Overview
2	Brief history
3	Branches
3.1	Dynamics
3.2	Statics
4	Relationship to classical mechanics
5	Examples
5.1	Newtonian Foundation
5.2	Archimedes' Principle
6	Major topics
6.1	Foundations and basic methods
6.2	Dynamics and vibration
6.3	Automatic control
6.4	Mechanics of solids
6.5	Mechanics of fluids
6.6	Thermal sciences
6.7	Earth sciences
6.8	Energy systems and environment
6.9	Biosciences
7	Applications
8	Publications
9	See also
10	References
11	Further reading
12	External links
Overview
Engineering problems are generally tackled with applied mechanics through the application of theories of classical mechanics and fluid mechanics[4]. Because applied mechanics can be applied in engineering disciplines like civil engineering, mechanical engineering, aerospace engineering, materials engineering, and biomedical engineering, it is sometimes referred to as engineering mechanics[4].

Science and engineering are interconnected with respect to applied mechanics, as researches in science are linked to research processes in civil, mechanical, aerospace, materials and biomedical engineering disciplines[1]. In civil engineering, applied mechanics’ concepts can be applied to structural design and a variety of engineering sub-topics like structural, coastal, geotechnical, construction, and earthquake engineering[4]. In mechanical engineering, it can be applied in mechatronics and robotics, design and drafting, nanotechnology, machine elements, structural analysis, friction stir welding, and acoustical engineering[4]. In aerospace engineering, applied mechanics is used in aerodynamics, aerospace structural mechanics and propulsion, aircraft design and flight mechanics[4]. ​​In materials engineering, applied mechanics’ concepts are used in thermoelasticity, elasticity theory, fracture and failure mechanisms, structural design optimisation, fracture and fatigue, active materials and composites, and computational mechanics[6]. Research in applied mechanics can be directly linked to biomedical engineering areas of interest like orthopaedics; biomechanics;  human body motion analysis; soft tissue modelling of muscles, tendons, ligaments, and cartilage; biofluid mechanics; and dynamic systems, performance enhancement, and optimal control[7].

Brief history
See also: History of classical mechanics and Timeline of classical mechanics

The first science with a theoretical foundation based in mathematics was mechanics; the underlying principles of mechanics were first delineated by Isaac Newton in his 1687 book Philosophiæ Naturalis Principia Mathematica[3]. One of the earliest works to define applied mechanics as its own discipline was the three volume Handbuch der Mechanik written by German physicist and engineer Franz Josef Gerstner[8]. The first seminal work on applied mechanics to be published in English was A Manual of Applied Mechanics in 1858 by English mechanical engineer William Rankine[8][9]. August Föppl, a German mechanical engineer and professor, published Vorlesungen über techische Mechanik in 1898 in which he introduced calculus to the study of applied mechanics[8].

Applied mechanics was established as a discipline separate from classical mechanics in the early 1920s with the publication of Journal of Applied Mathematics and Mechanics, the creation of the Society of Applied Mathematics and Mechanics, and the first meeting of the International Congress of Applied Mechanics[1].  In 1921 Austrian scientist Richard von Mises started the Journal of Applied Mathematics and Mechanics (Zeitschrift für Angewante Mathematik und Mechanik) and in 1922 with German scientist Ludwig Prandtl founded the Society of Applied Mathematics and Mechanics (Gesellschaft für Angewandte Mathematik und Mechanik)[1]. During a 1922 conference on hydrodynamics and aerodynamics in Innsbruck, Austria, Theodore von Kármán, a Hungarian engineer, and Tullio Levi-Civita, an Italian mathematician, met and decided to organize a conference on applied mechanics[1]. In 1924 the first meeting of the International Congress of Applied Mechanics was held in Delft, the Netherlands attended by more than 200 scientist from around the world[1][3].  Since this first meeting the congress has been held every four years, except during World War II; the name of the meeting was changed to International Congress of Theoretical and Applied Mechanics in 1960[1].

Due to the unpredictable political landscape in Europe after the First World War and upheaval of World War II many European scientist and engineers emigrated to the United States[1]. Ukrainian engineer Stephan Timoshenko fled the Bolsheviks Red Army in 1918 and eventually emigrated to the U.S. in 1922; over the next twenty-two years he taught applied mechanics at the University of Michigan and Stanford University[10]. Timoshenko authored thirteen textbooks in applied mechanics, many considered the gold standard in their fields; he also founded the Applied Mechanics Division of the American Society of Mechanical Engineers in 1927 and is considered “America’s Father of Engineering Mechanics.”[10] In 1930 Theodore von Kármán left Germany and became the first director of the Aeronautical Laboratory at the California Institute of Technology; von Kármán would later co-found the Jet Propulsion Laboratory in 1944[1]. With the leadership of Timoshenko and von Kármán, the influx of talent from Europe, and the rapid growth of the aeronautical and defense industries, applied mechanics became a mature discipline in the U.S. by 1950[1].

Branches
Dynamics
Main article: Dynamics

Dynamics, the study of the motion and movement of various objects, can be further divided into two branches, kinematics and kinetics[4]. For classical mechanics, kinematics would be the analysis of moving bodies using time, velocities, displacement, and acceleration[4]. Kinetics would be the study of moving bodies through the lens of the effects of forces and masses[4]. In the context of fluid mechanics, fluid dynamics pertains to the flow and describing of the motion of various fluids[4].

Statics
Main article: Statics

The study of statics is the study and describing of bodies at rest[4]. Static analysis in classical mechanics can be broken down into two categories, deformable bodies and non-deformable bodies[4]. When studying deformable bodies, considerations relating to the forces acting on the rigid structures are analyzed. When studying non-deformable bodies, the examination of the structure and material strength is observed[4]. In the context of fluid mechanics, the resting state of the pressure unaffected fluid is taken into account[4].

Relationship to classical mechanics
Applied Mechanics is a result of the practical applications of various engineering/mechanical disciplines; as illustrated in the table below[4].

Classical Mechanics/
Fluid Mechanics

Statics	Non-Deformable
Body

Practical
Applications

Civil
Engineering

Applied Mechanics
Deformable
Body

Mechanical
Engineering

Dynamics	Kinematics	Aerospace
Engineering

Kinetics	Materials
Engineering

Examples
Newtonian Foundation
Being one of the first sciences for which a systematic theoretical framework was developed, mechanics was spearheaded by Sir Isaac Newton's "Principia" (published in 1687)[3]. It is the "divide and rule" strategy developed by Newton that helped to govern motion and split it into dynamics or statics[3]. Depending on the type of force, type of matter, and the external forces, acting on said matter, will dictate the "Divide and Rule" strategy within dynamic and static studies[3].

Archimedes' Principle
Archimedes' principle is a major one that contains many defining propositions pertaining to fluid mechanics. As stated by proposition 7 of archimedes' principle, a solid that is heavier than the fluid its placed in, will descend to the bottom of the fluid[11]. If the solid is to be weighed within the fluid, the fluid will be measured as lighter than the weight of the amount of fluid that was displaced by said solid[11]. Further developed upon by proposition 5, if the solid is lighter than the fluid it is placed in, the solid will have to be forcibly immersed to be fully covered by the liquid[11]. The weight of the amount of displaced fluids will then be equal to the weight of the solid[11].</p>
                </div>
            </div>
        );
}

export default Taskview;