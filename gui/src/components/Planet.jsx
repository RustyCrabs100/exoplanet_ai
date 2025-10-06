import React from 'react'
import './Planet.css'

const Planet = () => {
  return (
    <div className="planet-container">
      <div className="planet">
        <div className="planet-surface">
          <svg className="surface-pattern" viewBox="0 0 200 200">
            <path 
              d="M40 60 Q60 40 80 60 T120 60" 
              stroke="#8B4513" 
              strokeWidth="2" 
              fill="none"
              opacity="0.6"
            />
            <path 
              d="M30 90 Q50 70 70 90 T110 90" 
              stroke="#8B4513" 
              strokeWidth="2" 
              fill="none"
              opacity="0.4"
            />
            <path 
              d="M60 120 Q80 100 100 120 T140 120" 
              stroke="#8B4513" 
              strokeWidth="2" 
              fill="none"
              opacity="0.5"
            />
            <path 
              d="M20 140 Q40 120 60 140 T100 140" 
              stroke="#8B4513" 
              strokeWidth="2" 
              fill="none"
              opacity="0.3"
            />
            <path 
              d="M80 160 Q100 140 120 160 T160 160" 
              stroke="#8B4513" 
              strokeWidth="2" 
              fill="none"
              opacity="0.4"
            />
            <path 
              d="M110 40 Q130 20 150 40 T190 40" 
              stroke="#8B4513" 
              strokeWidth="2" 
              fill="none"
              opacity="0.3"
            />
            <path 
              d="M140 80 Q160 60 180 80" 
              stroke="#8B4513" 
              strokeWidth="2" 
              fill="none"
              opacity="0.5"
            />
          </svg>
        </div>
      </div>
    </div>
  )
}

export default Planet
