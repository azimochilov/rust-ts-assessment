import { useRef, useState, useEffect } from 'react'
import './App.css'

function App() {
  const [text, setText] = useState('Ronaldo is the best.')
  const [selection, setSelection] = useState<{ start: number; end: number } | null>(null)
  const [loading, setLoading] = useState(false)
  const textareaRef = useRef<HTMLTextAreaElement>(null)

  useEffect(() => {
    const handleMouseUp = () => {
      const textarea = textareaRef.current
      if (!textarea) return

      const start = textarea.selectionStart
      const end = textarea.selectionEnd
      if (start !== end) {
        setSelection({ start, end })
      } else {
        setSelection(null)
      }
    }

    document.addEventListener('mouseup', handleMouseUp)
    return () => document.removeEventListener('mouseup', handleMouseUp)
  }, [])

  const handleParaphrase = async () => {
    if (!selection || !textareaRef.current) return

    const { start, end } = selection
    const selected = text.slice(start, end)

    setLoading(true)

    try {
      const res = await fetch(`https://ai-paraphraser-backend-pnp9.shuttle.app/paraphrase`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ text: selected })
      })

      const paraphrased = await res.text()

      if (!res.ok) {
        throw new Error(paraphrased)
      }

      const newText = text.slice(0, start) + paraphrased + text.slice(end)
      setText(newText)
      setSelection(null)
    } catch (err: unknown) {
      if (err instanceof Error) {
        console.error('Error:', err)
        alert('Failed to paraphrase: ' + err.message)
      } else {
        console.error('Unexpected error:', err)
        alert('An unknown error occurred while paraphrasing.')
      }
    } finally {
      setLoading(false)
    }
  }

  return (
    <div className="page">
      <div className="card">
        <h1>✍️ AI Paraphraser</h1>
        <textarea
          ref={textareaRef}
          value={text}
          onChange={(e) => setText(e.target.value)}
          placeholder="Paste or type text here. Select some of it to paraphrase."
        />
        {selection && (
          <button className="paraphrase-btn" onClick={handleParaphrase} disabled={loading}>
            {loading ? 'Paraphrasing...' : 'Paraphrase Selection'}
          </button>
        )}
      </div>
    </div>
  )
}

export default App
