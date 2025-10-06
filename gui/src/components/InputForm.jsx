import React from 'react'
import './InputForm.css'

const FIELDS = [
  { key: 'radius', label: 'Radius', placeholder: 'Earths' },
  { key: 'density', label: 'Density', placeholder: 'g/cm³' },
  { key: 'parsecs', label: 'Parsecs from Earth', placeholder: 'Parsecs' },
  { key: 'planetMass', label: 'Planet Mass', placeholder: 'Earths' },
  { key: 'vMagnitude', label: 'V (Johnson) Magnitude', placeholder: 'Magnitude' },
  { key: 'orbitalPeriod', label: 'Orbital Period', placeholder: 'Days' },
  { key: 'eccentricity', label: 'Eccentricity', placeholder: '' },
  { key: 'insolation', label: 'Insolation Flux', placeholder: 'Earth flux' },
  { key: 'eqTemp', label: 'Equilibrium Temperature', placeholder: 'Kelvin' },
  { key: 'stellarType', label: 'Stellar Type', placeholder: 'Spectral Type' },
  { key: 'stellarTeff', label: 'Stellar Effective Temp', placeholder: 'Kelvin' },
  { key: 'stellarRadius', label: 'Stellar Radius', placeholder: 'Solar Radius' },
  { key: 'stellarMass', label: 'Stellar Mass', placeholder: 'Solar Mass' },
  { key: 'stellarMetallicity', label: 'Stellar Metallicity', placeholder: 'dex' },
  { key: 'stellarGravity', label: 'Stellar Surface Gravity', placeholder: 'log10(cm/s²)' },
  { key: 'systemDistance', label: 'System Distance', placeholder: 'pc' },
  { key: 'systemVmag', label: 'System V Magnitude', placeholder: 'Magnitude' },
  { key: 'systemKmag', label: 'System Ks Magnitude', placeholder: 'Magnitude' },
  { key: 'systemGaiaMag', label: 'Gaia Magnitude', placeholder: 'Magnitude' },
  { key: 'ra', label: 'Right Ascension (RA)', placeholder: 'deg' },
  { key: 'dec', label: 'Declination (Dec)', placeholder: 'deg' },
  { key: 'hostName', label: 'Host Name', placeholder: 'Star Name' },
  { key: 'planetName', label: 'Planet Name', placeholder: 'Name' },
  { key: 'discoveryMethod', label: 'Discovery Method', placeholder: 'Method' }
]

const InputForm = ({ formData, onInputChange, onReset, onCalculate }) => {
  return (
    <div className="input-form">
      <div className="input-grid">
        {FIELDS.map(field => (
          <div className="input-group" key={field.key}>
            <label className="input-label">{field.label}</label>
            <input
              type="text"
              className="input-field"
              value={formData[field.key] || ''}
              onChange={e => onInputChange(field.key, e.target.value)}
              placeholder={field.placeholder}
            />
          </div>
        ))}
      </div>
      <div className="button-group">
        <span className="button-group-label">Button Group</span>
        <div className="buttons">
          <button className="btn btn-reset" type="button" onClick={onReset}>
            Reset
          </button>
          <button className="btn btn-calculate" type="button" onClick={onCalculate}>
            Calculate
          </button>
        </div>
      </div>
    </div>
  )
}

export default InputForm
