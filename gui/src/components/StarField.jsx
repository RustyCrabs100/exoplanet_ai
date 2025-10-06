import React, { useEffect, useRef } from 'react'
import './StarField.css'

const StarField = () => {
  const canvasRef = useRef(null)

  useEffect(() => {
    const canvas = canvasRef.current
    const ctx = canvas.getContext('2d')
    
    const resizeCanvas = () => {
      canvas.width = window.innerWidth
      canvas.height = window.innerHeight
    }
    
    resizeCanvas()
    window.addEventListener('resize', resizeCanvas)

    // Generate stars
    const stars = []
    const numStars = 200

    for (let i = 0; i < numStars; i++) {
      stars.push({
        x: Math.random() * canvas.width,
        y: Math.random() * canvas.height,
        size: Math.random() * 3 + 1,
        opacity: Math.random() * 0.8 + 0.2,
        twinkleSpeed: Math.random() * 0.02 + 0.01
      })
    }

    // Generate bright stars with sparkle effect
    const brightStars = []
    const numBrightStars = 8

    for (let i = 0; i < numBrightStars; i++) {
      brightStars.push({
        x: Math.random() * canvas.width,
        y: Math.random() * canvas.height,
        size: Math.random() * 2 + 3,
        opacity: 1,
        sparkle: 0
      })
    }

    let animationId
    let time = 0

    const animate = () => {
      ctx.clearRect(0, 0, canvas.width, canvas.height)
      time += 0.01

      // Draw regular stars
      stars.forEach(star => {
        star.opacity = 0.3 + Math.sin(time * star.twinkleSpeed + star.x) * 0.5
        ctx.globalAlpha = Math.max(0.1, star.opacity)
        ctx.fillStyle = 'white'
        ctx.beginPath()
        ctx.arc(star.x, star.y, star.size, 0, Math.PI * 2)
        ctx.fill()
      })

      // Draw bright stars with sparkle effect
      brightStars.forEach(star => {
        ctx.globalAlpha = 1
        ctx.fillStyle = 'white'
        
        // Draw the star
        ctx.beginPath()
        ctx.arc(star.x, star.y, star.size, 0, Math.PI * 2)
        ctx.fill()

        // Draw sparkle lines
        const sparkleLength = 15 + Math.sin(time * 2 + star.x) * 5
        ctx.strokeStyle = 'white'
        ctx.lineWidth = 1
        ctx.globalAlpha = 0.8 + Math.sin(time * 3 + star.y) * 0.2

        // Horizontal line
        ctx.beginPath()
        ctx.moveTo(star.x - sparkleLength, star.y)
        ctx.lineTo(star.x + sparkleLength, star.y)
        ctx.stroke()

        // Vertical line
        ctx.beginPath()
        ctx.moveTo(star.x, star.y - sparkleLength)
        ctx.lineTo(star.x, star.y + sparkleLength)
        ctx.stroke()

        // Diagonal lines
        const diagLength = sparkleLength * 0.7
        ctx.beginPath()
        ctx.moveTo(star.x - diagLength, star.y - diagLength)
        ctx.lineTo(star.x + diagLength, star.y + diagLength)
        ctx.stroke()

        ctx.beginPath()
        ctx.moveTo(star.x - diagLength, star.y + diagLength)
        ctx.lineTo(star.x + diagLength, star.y - diagLength)
        ctx.stroke()
      })

      ctx.globalAlpha = 1
      animationId = requestAnimationFrame(animate)
    }

    animate()

    return () => {
      window.removeEventListener('resize', resizeCanvas)
      cancelAnimationFrame(animationId)
    }
  }, [])

  return <canvas ref={canvasRef} className="star-field" />
}

export default StarField
