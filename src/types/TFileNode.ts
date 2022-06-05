type TFileNode = {
  last_modified: Date | null
  is_folder: boolean
  name: string
  s3_key: string
  size: number
}

export const defaultFolder: TFileNode = {
  last_modified: null,
  is_folder: true,
  name: '',
  s3_key: '',
  size: 0,
}

export default TFileNode
