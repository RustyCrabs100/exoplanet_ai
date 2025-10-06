import React from 'react'
import './InputForm.css'

const InputForm = ({ formData, onInputChange, onReset, onCalculate }) => {
  const handleInputChange = (field, value) => {
    onInputChange(field, value)
  }

  return (
    <div className="input-form">
      <div className="input-grid">
        <div className="input-group">
          <label className="input-label">Radius</label>
          <input
            type="text"
            className="input-field"
            value={formData.radius}
            onChange={(e) => handleInputChange('radius', e.target.value)}
            placeholder="Earths"
          />
        </div>

        <div className="input-group">
          <label className="input-label">Density</label>
          <input
            type="text"
            className="input-field"
            value={formData.density}
            onChange={(e) => handleInputChange('density', e.target.value)}
            placeholder="g/cm**3"
          />
        </div>

        <div className="input-group">
          <label className="input-label">Parsecs from Earth</label>
          <input
            type="text"
            className="input-field"
            value={formData.parsecs}
            onChange={(e) => handleInputChange('parsecs', e.target.value)}
            placeholder="Parsecs"
          />
        </div>

        <div className="input-group">
          <label className="input-label">Planet mass</label>
          <input
            type="text"
            className="input-field"
            value={formData.planetMass}
            onChange={(e) => handleInputChange('planetMass', e.target.value)}
            placeholder="Earths"
          />
        </div>

        <div className="input-group">
          <label className="input-label">V (Johnson) magnitude</label>
          <input
            type="text"
            className="input-field"
            value={formData.vMagnitude}
            onChange={(e) => handleInputChange('vMagnitude', e.target.value)}
            placeholder="Magnitude"
          />
        </div>

        <div className="input-group">
          <label className="input-label">Orbital Period</label>
          <input
            type="text"
            className="input-field"
            value={formData.orbitalPeriod}
            onChange={(e) => handleInputChange('orbitalPeriod', e.target.value)}
            placeholder="Days"
          />
        </div>
      </div>

      <div className="button-group">
        <span className="button-group-label">Button Group</span>
        <div className="buttons">
          <button className="btn btn-reset" onClick={onReset}>
            Reset
          </button>
          <button className="btn btn-calculate" onClick={onCalculate}>
            Calculate
          </button>
        </div>
      </div>
    </div>
  )
}

export default InputForm
