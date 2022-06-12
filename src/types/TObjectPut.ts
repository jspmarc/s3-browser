import type TObjectAcl from './TObjectAcl'

type TObjectPut = {
  path: string
  key: string
  acl: TObjectAcl
}

export default TObjectPut
