import React, { useState } from 'react'
import StarField from './components/StarField'
import Planet from './components/Planet'
import InputForm from './components/InputForm'
import './App.css'

function App() {
  const [formData, setFormData] = useState({
    radius: 'Earths',
    density: 'g/cm**3',
    parsecs: 'Parsecs',
    planetMass: 'Earths',
    vMagnitude: 'Magnitude',
    orbitalPeriod: 'Days'
  })

  const handleInputChange = (field, value) => {
    setFormData(prev => ({
      ...prev,
      [field]: value
    }))
  }

  const handleReset = () => {
    setFormData({
      radius: 'Earths',
      density: 'g/cm**3',
      parsecs: 'Parsecs',
      planetMass: 'Earths',
      vMagnitude: 'Magnitude',
      orbitalPeriod: 'Days'
    })
  }

  const handleCalculate = () => {
    console.log('Calculating with data:', formData)
  }

  return (
    <div className="app">
      <StarField />
      <div className="content">
        <header className="header">
          <h1 className="title">RUSTY PLANET FINDER</h1>
          <h2 className="version">V 0.1</h2>
        </header>
        
        <div className="main-content">
          <Planet />
          <InputForm 
            formData={formData}
            onInputChange={handleInputChange}
            onReset={handleReset}
            onCalculate={handleCalculate}
          />
        </div>
      </div>
    </div>
  )
}

export default App
