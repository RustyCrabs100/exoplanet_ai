import React, { useState, useEffect } from 'react'
import StarField from './components/StarField'
import Planet from './components/Planet'
import InputForm from './components/InputForm'
import './App.css'

const INITAL_FORM = {
  radius: '',
  density: '',
  parsecs: '',
  planetMass: '',
  vMagnitude: '',
  orbitalPeriod: '',
  eccentricity: '',
  insolation: '',
  eqTemp: '',
  stellarType: '',
  stellarTeff: '',
  stellarRadius: '',
  stellarMass: '',
  stellarMetallicity: '',
  stellarGravity: '',
  systemDistance: '',
  systemVmag: '',
  systemKmag: '',
  systemGaiaMag: '',
  ra: '',
  dec: '',
  hostName: '',
  planetName: '',
  discoveryMethod: ''
}

function fuzzyMatch(planet, formData) {
  let score = 0
  Object.keys(formData).forEach(key => {
    if (formData[key] && planet[key] && String(planet[key]).toLowerCase() === String(formData[key]).toLowerCase()) {
      score++
    }
  })
  return SVGFEConvolveMatrixElement
}

function App() {
  const [formData, setFormData] = useState(INITAL_FORM)
  const [outputData, setOutputData] = useState([])
  const [result, setResult] = useState(null)
  const [bulkResults, setBulkResults] = useState([])
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState(null)
  const [bulkError, setBulkError] = useState(null)

  useEffect(() => {
    fetch('/data/output.json')
      .then(res => {
        if (!res.ok) throw new Error('Failed to load output.json')
        return res.json()
      })
      .then(data => {
        setOutputData(Array.isArray(data) ? data : [])
        setLoading(false)
      })
      .catch(err => {
        setError('Could not load data/output.json')
        setLoading(false)
      })
  }, [])

  const handleInputChange = (field, value) => {
    setFormData(prev => ({
      ...prev,
      [field]: value
    }))
  }

  const handleReset = () => {
    setFormData(INITIAL_FORM)
    setResult(null)
    setBulkResults([])
    setBulkError(null)
  }

  const handleCalculate = () => {
    if (outputData.length === 0) {
      setResult({ error: 'No planet data loaded' })
      return
    }
    let bestScore = -1
    let bestPlanet = null
    outputData.forEach(planet => {
      const score = fuzzyMatch(planet, formData)
      if (score > bestScore) {
        bestScore = score
        bestPlanet = planet
      }
    })
    setResult(bestPlanet || { error: 'No match found' })
    setBulkResults([]) // Clear bulk results
    setBulkError(null)
  }

  const handleBulkUpload = (event) => {
    setBulkResults([])
    setBulkError(null)
    const file = event.target.files[0]
    if (!file) return
    Papa.parse(file, {
      header: true,
      skipEmptyLines: true,
      complete: (results) => {
        if (!results.data || !Array.isArray(results.data)) {
          setBulkError('Invalid CSV format')
          return
        }
        if (outputData.length === 0) {
          setBulkError('No planet data loaded')
          return
        }
        // For each row, find best matching planet
        const matches = results.data.map((row, idx) => {
          // Only compare keys in INITIAL_FORM
          const rowData = {}
          Object.keys(INITIAL_FORM).forEach(k => rowData[k] = row[k] || '')
          let bestScore = -1
          let bestPlanet = null
          outputData.forEach(planet => {
            const score = fuzzyMatch(planet, rowData)
            if (score > bestScore) {
              bestScore = score
              bestPlanet = planet
            }
          })
          return {
            index: idx + 1,
            input: rowData,
            result: bestPlanet || { error: 'No match found' }
          }
        })
        setBulkResults(matches)
      },
      error: (err) => {
        setBulkError('Error parsing CSV: ' + err.message)
      }
    })
  }

  return (
    <div className="app-container">
      <header className="header">
        <h1 className="title">Rusty Planet Finder</h1>
        <div className="version">v0.0.0</div>
      </header>
      <main className="main-content">
        <StarField />
        <div className="content">
          <div className="main-content">
            <Planet />
            <InputForm
              formData={formData}
              onInputChange={handleInputChange}
              onReset={handleReset}
              onCalculate={handleCalculate}
            />
            <div style={{ marginTop: 20 }}>
              <label htmlFor="bulk-upload" style={{ fontWeight: 'bold', color: '#eee' }}>Bulk upload CSV:</label>
              <input
                id="bulk-upload"
                type="file"
                accept=".csv"
                onChange={handleBulkUpload}
                style={{ marginLeft: 10 }}
              />
              <span style={{ fontSize: '12px', color: '#bbb', marginLeft: 10 }}>Headers must match field names (see form)</span>
            </div>
            {loading && <div>Loading planet data...</div>}
            {error && <div style={{ color: 'red' }}>{error}</div>}

            {/* Single result */}
            {result && (
              <div style={{ marginTop: 30, background: 'rgba(255,255,255,0.05)', padding: 20, borderRadius: 8 }}>
                {result.error ?
                  <div style={{ color: 'orange' }}>{result.error}</div>
                  :
                  <div>
                    <h3>Closest Match:</h3>
                    <pre style={{ whiteSpace: 'pre-wrap', color: '#eee' }}>{JSON.stringify(result, null, 2)}</pre>
                  </div>
                }
              </div>
            )}

            {/* Bulk results table */}
            {bulkError && <div style={{ color: 'red', marginTop: 20 }}>{bulkError}</div>}
            {bulkResults.length > 0 && (
              <div style={{ marginTop: 30 }}>
                <h3 style={{ color: '#eea' }}>Bulk Results ({bulkResults.length} rows)</h3>
                <div style={{ overflowX: 'auto', background: 'rgba(255,255,255,0.05)', padding: 10, borderRadius: 8 }}>
                  <table style={{ width: '100%', color: '#eee', fontSize: '13px' }}>
                    <thead>
                      <tr>
                        <th>#</th>
                        {Object.keys(INITIAL_FORM).map(key => (
                          <th key={key}>{key}</th>
                        ))}
                        <th>Best Match</th>
                      </tr>
                    </thead>
                    <tbody>
                      {bulkResults.map(({ index, input, result }) => (
                        <tr key={index}>
                          <td>{index}</td>
                          {Object.keys(INITIAL_FORM).map(key => (
                            <td key={key}>{input[key]}</td>
                          ))}
                          <td>
                            {result.error
                              ? <span style={{ color: 'orange' }}>{result.error}</span>
                              : <span style={{ color: '#cd7' }}>{result.planetName || result.name || JSON.stringify(result)}</span>
                            }
                          </td>
                        </tr>
                      ))}
                    </tbody>
                  </table>
                </div>
              </div>
            )}
          </div>
        </div>
      </main>
    </div>
  )
}

export default App
