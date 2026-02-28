'use client'

import { useCallback } from 'react'
import { Editor } from '@tldraw/tldraw'
import { toast } from 'react-hot-toast'

interface UseFileUploadProps {
  editor: Editor | null
}

export function useFileUpload({ editor }: UseFileUploadProps) {
  const uploadFile = useCallback(async (file: File, x: number, y: number) => {
    if (!editor) return

    const formData = new FormData()
    formData.append('file', file)
    formData.append('position_x', x.toString())
    formData.append('position_y', y.toString())

    try {
      const response = await fetch('http://localhost:8080/api/v1/upload', {
        method: 'POST',
        body: formData,
      })

      if (!response.ok) {
        throw new Error('Upload failed')
      }

      const node = await response.json()
      
      // Update tldraw canvas with the new node
      // For now, we'll use a placeholder shape or a custom shape if defined
      editor.createShapes([
        {
          id: `shape:${node.id}` as any,
          type: 'geo',
          x,
          y,
          props: {
            geo: 'rectangle',
            w: 200,
            h: 150,
            text: file.name,
            color: 'black',
            fill: 'pattern',
          },
        },
      ])

      toast.success(`${file.name} hochgeladen!`)
    } catch (error) {
      console.error('Error uploading file:', error)
      toast.error(`Fehler beim Upload von ${file.name}`)
    }
  }, [editor])

  const handleDrop = useCallback(async (e: React.DragEvent) => {
    e.preventDefault()
    if (!editor) return

    const files = Array.from(e.dataTransfer.files)
    if (files.length === 0) return

    const { x, y } = editor.inputs.currentPagePoint

    for (const file of files) {
      await uploadFile(file, x, y)
    }
  }, [editor, uploadFile])

  return { handleDrop, uploadFile }
}
