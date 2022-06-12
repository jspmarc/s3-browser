import TFileNode from './TFileNode'
import TObjectsList from './TObjectsList'

type TStore = {
  baseUrl: string
  keys: TFileNode[]
  objects: Map<string, TObjectsList>
}

export default TStore
