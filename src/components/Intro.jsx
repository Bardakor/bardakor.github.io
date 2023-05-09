import React from "react";

function Intro() {
    return (
        <div className="flex items-center justify-center flex-col text-center pt-20 pb-6">
            <h1 className="text-4xl md:text-7xl dark:text-white mb-1 md:mb-3 font-bold">Liam</h1>
            <p className="text-base md:text-xl mb-3 font-medium">Computer Science Student & Software Engineer</p>
            <div className="max-w-xl text-sm mb-6">
                <p className="mb-3 font-bold">As a student of Computer Science at Efrei in Paris, I am passionate about the intersection of technology and sports. My goal is to work in the sports industry, specifically in the realm of data and statistics analysis.</p>
                <p className="mb-3 font-bold">My love for football stems from my experience as a former player at Arsenal during my teenage years. I am also an accomplished software developer, having designed several programs. You can check out my projects on my <a href="https://github.com/Bardakor" target="_blank" rel="noopener noreferrer" className="text-blue-600 hover:text-green-500 transition duration-300">GitHub page</a>.</p>
            </div>
        </div>


    )
}

export default Intro;