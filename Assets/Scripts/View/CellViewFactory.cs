using UnityEngine;
using UnityEngine.AddressableAssets;
using UnityEngine.ResourceManagement.AsyncOperations;

namespace View
{
    public class CellViewFactory
    {
        private const string CellViewPrefabPath = "Cell";

        private AsyncOperationHandle<GameObject> _prefabHandle;

        public CellViewFactory()
        {
            _prefabHandle = Addressables.LoadAssetAsync<GameObject>(CellViewPrefabPath);
            _prefabHandle.WaitForCompletion();
        }

        public CellView Create(Vector2Int index)
        {
            var cellView = Object.Instantiate(_prefabHandle.Result).GetComponent<CellView>();
            cellView.Init(index, CellViewType.Empty);
            cellView.transform.position = new Vector3(index.x, 0, index.y);
            return cellView;
        }
    }
}